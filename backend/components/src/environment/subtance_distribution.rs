use crate::environment::coordinate::Coordinate;
use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::env_dynamic::t_env_dynamic::EnvDynamic;
use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use crate::environment::map_size::MapSize;
use crate::environment::noise_params::NoiseParams;
use crate::environment::t_noise_generatable::NoiseGeneratable;
use crate::environment::t_statistical::Statistical;
use crate::shared::property::Property;
use crate::shared::subtance_type::SubtanceType;
use ndarray::{Array2, Zip};
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::instrument;

use super::hexagon::t_hexa_relational::HexaRelational;

const ENLARGE_FACTOR: usize = 255;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SubtanceDistribution {
    pub resource_type: SubtanceType,
    pub distribution: Array2<usize>,
    pub noise_params: NoiseParams,
}

impl SubtanceDistribution {
    #[instrument(skip_all)]
    pub fn new(
        resource_type: SubtanceType,
        map_size: MapSize,
        noise_params: Option<NoiseParams>,
    ) -> Self {
        let noise_params = noise_params.unwrap_or_default(); // 使用 `NoiseParams` 的默认值
        let distribution = Array2::zeros(map_size.as_tuple());

        Self {
            resource_type,
            distribution,
            noise_params,
        }
    }

    fn potential_based_diffuse(&mut self, potential: &Array2<f64>) {
        // 创建一个新的 Array2
        let dimensions = self.distribution.dim();
        let new_distribution = Arc::new(Mutex::new(Array2::zeros(dimensions)));

        // 使用 Zip 并行遍历 distribution，并将计算后的值存储到 new_distribution
        Zip::indexed(&self.distribution).par_for_each(|(y, x), _| {
            // 通过索引 (y, x) 计算新的值
            let center = Coordinate::new(y, x);
            let neighbours = center.get_relations_map::<NeighbourRelation>(dimensions);

            let mut new_distribution = new_distribution.lock().unwrap();
            Self::calculate_diffusion_for_position(
                center,
                neighbours,
                potential,
                &mut new_distribution,
            );
        });

        // 提取出 new_distribution 的结果
        let new_distribution = Arc::try_unwrap(new_distribution)
            .expect("无法解包 Arc")
            .into_inner()
            .expect("无法解锁 Mutex");

        // 将计算结果存储到 self.distribution 中
        self.distribution = new_distribution;
    }

    fn calculate_diffusion_for_position(
        center: Coordinate,
        neighbours: HashMap<NeighbourRelation, Coordinate>,
        potential: &Array2<f64>,
        new_distribution: &mut Array2<usize>,
    ) -> usize {
        let center_potential = potential[[*center.y(), *center.x()]];

        // 合并计算每个邻居的势能差异和梯度总和
        let total_gradient = neighbours
            .iter()
            .map(|(relation, nb_coordinate)| {
                // 计算邻居的势能差异
                let neighbour_potential = potential[[*nb_coordinate.y(), *nb_coordinate.x()]];
                let disparity = neighbour_potential - center_potential;

                // 获取邻居坐标的偏移，作为势能梯度的方向
                let coordinate_shift =
                    NeighbourRelation::from_relation_to_coordinate_shift()[relation];

                // 势能差异和方向偏移相乘，得到梯度方向上的势能差异
                disparity * coordinate_shift
            })
            .sum::<CoordinateShift>();

        total_gradient // 这里可以返回 `total_gradient` 或继续进一步计算
    }
}

impl NoiseGeneratable for SubtanceDistribution {
    #[instrument(skip_all)]
    fn generate_simplex_noise(&mut self) {
        // 初始化Simplex噪声生成器，使用指定的种子确保噪声的可重复性
        let simplex = OpenSimplex::new(self.noise_params.seed);
        // 设置噪声频率，控制噪声的扩展和分布范围
        let frequency = self.noise_params.scale * 2.0 * std::f64::consts::PI;

        let width = self.distribution.shape()[1];
        let height = self.distribution.shape()[0];

        // 使用并行处理来加速矩阵中每个元素的噪声计算
        Zip::indexed(&mut self.distribution).par_for_each(|(row_index, col_index), value| {
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
            *value = ((noise_value + 1.0) * 0.5 * ENLARGE_FACTOR as f64) as usize;
        });
    }
}

impl Statistical for SubtanceDistribution {
    type Item = usize;

    #[instrument(skip_all)]
    fn min(&self) -> Self::Item {
        *self.distribution.par_iter().min().expect("分布为空")
    }

    #[instrument(skip_all)]
    fn max(&self) -> Self::Item {
        *self.distribution.par_iter().max().expect("分布为空")
    }

    #[instrument(skip_all)]
    fn mean(&self) -> f64 {
        let sum: usize = self.distribution.par_iter().sum();
        sum as f64 / self.distribution.len() as f64
    }

    #[instrument(skip_all)]
    fn variance(&self) -> f64 {
        let mean = self.mean();
        let sum_of_squares: f64 = self
            .distribution
            .par_iter()
            .map(|&value| {
                let diff = value as f64 - mean;
                diff * diff
            })
            .sum();

        sum_of_squares / (self.distribution.len() - 1) as f64
    }
}

impl EnvDynamic for SubtanceDistribution {
    type StateType = Array2<usize>;

    #[instrument(skip_all)]
    fn update(&mut self) {
        self.generate_simplex_noise();
    }

    #[instrument(skip_all)]
    fn state(&self) -> &Array2<usize> {
        &self.distribution
    }
}
