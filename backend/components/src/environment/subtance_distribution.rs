use crate::environment::coordinate::Coordinate;
use crate::environment::diffuse_info::DiffuseInfo;
use crate::environment::hexagon::hex_block::HexBlock;
use crate::environment::hexagon::hex_unit::HexUnit;
use crate::environment::hexagon::indexed_unit_change::IndexedUnitChange;
use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use crate::environment::hexagon::unit_change::UnitChange;
use crate::environment::map_size::MapSize;
use crate::environment::noise_params::NoiseParams;
use crate::environment::potential::Potential;
use crate::environment::t_indexed::Indexed;
use crate::environment::t_noise_generatable::NoiseGeneratable;
use crate::environment::t_statistical::Statistical;
use crate::shared::subtance_type::SubstanceType;
use ndarray::{Array2, Zip};
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;
use serde::Serialize;
use std::array;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use tracing::instrument;

const ENLARGE_FACTOR: usize = 255;

#[derive(Debug, Clone, Serialize, Eq)]
pub(crate) struct SubstanceDistribution {
    substance_type: SubstanceType,
    distribution: Array2<HexUnit>,
    noise_params: NoiseParams,
}

impl SubstanceDistribution {
    #[instrument(skip_all)]
    pub(crate) fn new(
        substance_type: SubstanceType,
        map_size: MapSize,
        noise_params: Option<NoiseParams>,
    ) -> Self {
        let noise_params = noise_params.unwrap_or_default(); // 使用 `NoiseParams` 的默认值
        let distribution = Array2::from_elem(map_size.as_tuple(), HexUnit::default());

        Self {
            substance_type,
            distribution,
            noise_params,
        }
    }

    pub(crate) fn substance_type(&self) -> &SubstanceType {
        &self.substance_type
    }

    pub(crate) fn distribution(&self) -> &Array2<HexUnit> {
        &self.distribution
    }

    pub(crate) fn noise_params(&self) -> &NoiseParams {
        &self.noise_params
    }

    /// 对整个网格执行扩散逻辑的主函数。
    /// 1. 使用 `compute_changes` 并行计算所有单元格及其邻居的变化量。
    /// 2. 使用 `apply_changes` 串行地将变化量应用到分布中，从而更新每个单元格的状态。
    pub(crate) fn diffuse(&mut self, now_potential: &Potential) {
        // 1. 并行计算变化量：获取每个格子和其邻居的变化结果。
        let changes = self.compute_changes(now_potential);

        // 2. 串行应用变化量：将变化写入 self.distribution 中，完成分布的更新。
        self.apply_changes(changes);
    }

    /// 并行计算所有单元格及其邻居的变化量。
    ///
    /// 返回值是一个 `Vec<(IndexedUnitChange, [IndexedUnitChange; 6])>`:
    /// - `IndexedUnitChange`：表示中心单元格的变化量以及其行列坐标。
    /// - `[IndexedUnitChange; 6]`：表示该中心格子6个邻居的变化量（固定数量），
    ///   其中每个 `IndexedUnitChange` 中也包含行列坐标和变化信息。
    ///
    /// 整个过程：
    /// - 使用 `indexed_iter()` 获取分布中的每个单元格及其索引 `(row_index, col_index)`。
    /// - `par_bridge()` 将普通迭代器转换为并行迭代器，在多核环境下并行处理每个格子。
    /// - 对每个单元格构造 `HexBlock<DiffuseInfo>`（中心+邻居），调用 `old_unit.diffuse(...)` 获得 `HexBlock<UnitChange>`.
    /// - 将结果组装为 `(IndexedUnitChange, [IndexedUnitChange; 6])` 返回。
    fn compute_changes(
        &self,
        now_potential: &Potential,
    ) -> Vec<(IndexedUnitChange, [IndexedUnitChange; 6])> {
        self.distribution
            .indexed_iter()
            .par_bridge() // 并行化处理每个单元格
            .map(|((row_index, col_index), old_unit)| {
                // 为当前单元格构造扩散所需的上下文信息块
                let (block_of_info, relations_map) =
                    self.build_hex_block_of_info(row_index, col_index, now_potential, old_unit);

                // 调用当前单元格的扩散方法，得到中心和邻居的变化量（HexBlock<UnitChange>）
                let block_of_change = old_unit.diffuse(&block_of_info);

                // 构建中心格子的变化信息
                let center_change =
                    IndexedUnitChange::new(row_index, col_index, *block_of_change.center());

                // 将邻居变化量从 HashMap 转换为一个固定长度数组 [IndexedUnitChange; 6]
                // 我们假定有且仅有6个邻居，顺序由迭代枚举 (enumerate) 来保持一致。
                let neighbour_changes: [IndexedUnitChange; 6] = array::from_fn(|i| {
                    let (relation, unit_change) = block_of_change
                        .neighbors()
                        .iter()
                        .nth(i) // 使用索引 i 获取第 i 个元素
                        .expect("邻居关系缺失！");
                    let neighbour_coord = relations_map.get(relation).expect("邻居关系缺失！");
                    IndexedUnitChange::new(neighbour_coord.y(), neighbour_coord.x(), *unit_change)
                });

                // 返回中心变化和6个邻居变化
                (center_change, neighbour_changes)
            })
            .collect()
    }

    /// 根据给定的行列索引和当前势能，构建包含中心和邻居信息的 HexBlock<DiffuseInfo>。
    ///
    /// 返回：
    /// - `HexBlock<DiffuseInfo>`：中心单元+邻居单元的势能和状态信息构成的上下文块，用于扩散计算。
    /// - `HashMap<NeighbourRelation, Coordinate>`：邻居关系到坐标的映射表，供后续获取邻居坐标使用。
    fn build_hex_block_of_info(
        &self,
        row_index: usize,
        col_index: usize,
        now_potential: &Potential,
        old_unit: &HexUnit,
    ) -> (
        HexBlock<DiffuseInfo>,
        HashMap<NeighbourRelation, Coordinate>,
    ) {
        // 当前格子的坐标
        let current_coord = Coordinate::new(row_index, col_index);

        // 获取邻居关系与坐标的映射表（如 Relation::Degree60 -> (row, col)）
        let relations_map = current_coord.get_relations_map::<NeighbourRelation>();

        // 构建邻居单元的 DiffuseInfo Map
        // DiffuseInfo 包含邻居单元的状态和它的势能
        let neighbors_map: HashMap<NeighbourRelation, DiffuseInfo> = relations_map
            .iter()
            .map(|(relation, neighbour_coord)| {
                // 获取邻居单元格状态
                let neighbour_unit = self
                    .distribution
                    .get([neighbour_coord.y(), neighbour_coord.x()])
                    .expect("邻居单元格越界");

                // 获取邻居单元的势能值
                let neighbour_potential = now_potential
                    .distribution()
                    .get([neighbour_coord.y(), neighbour_coord.x()])
                    .expect("邻居势能分布越界");

                // 构造邻居的 DiffuseInfo
                (
                    *relation,
                    DiffuseInfo::new(*neighbour_unit, *neighbour_potential),
                )
            })
            .collect();

        // 中心单元的势能值
        let center_potential = now_potential
            .distribution()
            .get([row_index, col_index])
            .expect("中心单元势能分布越界");
        let center_info = DiffuseInfo::new(*old_unit, *center_potential);

        // 构建HexBlock：由中心信息和邻居信息共同组成扩散所需的上下文
        let block_of_info = HexBlock::new(center_info, neighbors_map);

        (block_of_info, relations_map)
    }

    /// 将并行计算得到的变化量应用到 distribution 上，从而完成对所有单元格状态的更新。
    ///
    /// 参数：
    /// - `changes`: 来自 `compute_changes` 的返回值，一个包含多组 (中心变化, 邻居变化数组) 的列表。
    ///
    /// 流程：
    /// 1. 使用 `fold` 将所有变化量汇总到 `change_dist` 数组中。
    ///    `change_dist` 是一个与 `self.distribution` 同尺寸的二维数组，每个元素是 `UnitChange` 的累积。
    /// 2. 遍历 `change_dist` 与 `self.distribution`，将累积的变化量应用到实际的单元格中。
    fn apply_changes(&mut self, changes: Vec<(IndexedUnitChange, [IndexedUnitChange; 6])>) {
        // 初始化 change_dist 为与 distribution 同大小的 UnitChange 数组，全部默认值
        let change_dist = changes.iter().fold(
            Array2::from_elem(self.distribution.dim(), UnitChange::default()),
            |mut acc, (center_change, neighbour_changes)| {
                // 应用中心变化量到 acc 中指定坐标处
                acc[[center_change.y(), center_change.x()]]
                    .accumulate_change(center_change.change());

                // 应用邻居变化量
                for nb_change in neighbour_changes.iter() {
                    acc[[nb_change.y(), nb_change.x()]].accumulate_change(nb_change.change());
                }

                acc
            },
        );

        // 将所有计算出的变更应用到 self.distribution
        // 使用 Zip 将 distribution 与 change_dist 对应位置打包在一起
        Zip::from(&mut self.distribution)
            .and(&change_dist)
            .for_each(|unit, change| {
                // 每个单元格应用对应的变化量
                unit.fit_change(*change);
            });
    }
}

// 自定义 Hash
impl Hash for SubstanceDistribution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.substance_type.hash(state); // 仅使用 substance_type 的哈希值
    }
}

// 自定义 PartialEq
impl PartialEq for SubstanceDistribution {
    fn eq(&self, other: &Self) -> bool {
        self.substance_type == other.substance_type // 仅比较 substance_type
    }
}

impl NoiseGeneratable for SubstanceDistribution {
    #[instrument(skip_all)]
    fn generate_simplex_noise(&mut self) {
        // 初始化Simplex噪声生成器，使用指定的种子确保噪声的可重复性
        let simplex = OpenSimplex::new(self.noise_params.seed);
        // 设置噪声频率，控制噪声的扩展和分布范围
        let frequency = self.noise_params.scale * 2.0 * std::f64::consts::PI;

        let width = self.distribution.shape()[1];
        let height = self.distribution.shape()[0];

        // 使用并行处理来加速矩阵中每个元素的噪声计算
        Zip::indexed(&mut self.distribution).par_for_each(|(row_index, col_index), unit| {
            // 计算当前元素在矩阵中的归一化坐标，确保噪声在整个地图范围内分布均匀
            let normalized_colidx = col_index as f64 / width as f64;
            let normalized_rowidx = row_index as f64 / height as f64;

            // 将二维平面坐标映射到四维周期性空间，生成环绕噪声
            // 通过sin和cos创建周期性，确保在边界处噪声无缝连接，实现地图的平滑环绕效果
            let s = (normalized_colidx * frequency).sin(); // 横向环绕的sin分量
            let c = (normalized_colidx * frequency).cos(); // 横向环绕的cos分量
            let t = (normalized_rowidx * frequency).sin(); // 纵向环绕的sin分量
            let u = (normalized_rowidx * frequency).cos(); // 纵向环绕的cos分量

            // 生成噪声值，基于四维空间中的坐标，确保噪声在整个地图上连续
            // 归一化噪声值到[0, 255]范围，便于后续使用或显示
            let noise_value = simplex.get([s, c, t, u]);
            unit.set_mole(((noise_value + 1.0) * 0.5 * ENLARGE_FACTOR as f64) as usize);
        });
    }
}

impl Statistical for SubstanceDistribution {
    type Item = HexUnit;

    #[instrument(skip_all)]
    fn min(&self) -> Self::Item {
        self.distribution
            .par_iter()
            .min_by_key(|unit| unit.mole())
            .expect("分布为空")
            .clone()
    }

    #[instrument(skip_all)]
    fn max(&self) -> Self::Item {
        self.distribution
            .par_iter()
            .max_by_key(|unit| unit.mole())
            .expect("分布为空")
            .clone()
    }

    #[instrument(skip_all)]
    fn mean(&self) -> f64 {
        let sum: usize = self.distribution.par_iter().map(|unit| unit.mole()).sum();
        sum as f64 / self.distribution.len() as f64
    }

    #[instrument(skip_all)]
    fn variance(&self) -> f64 {
        let mean = self.mean();
        let sum_of_squares: f64 = self
            .distribution
            .par_iter()
            .map(|unit| {
                let diff = unit.mole() as f64 - mean;
                diff * diff
            })
            .sum();

        sum_of_squares / (self.distribution.len() - 1) as f64
    }
}
