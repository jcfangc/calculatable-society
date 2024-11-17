// use crate::environment::diffusion_engine::DiffusionEngine;
// use crate::environment::env_dynamic::t_env_dynamic::EnvDynamic;
use crate::environment::hexagon::hex_unit::HexUnit;
use crate::environment::map_size::MapSize;
use crate::environment::noise_params::NoiseParams;
use crate::environment::t_noise_generatable::NoiseGeneratable;
use crate::environment::t_statistical::Statistical;
use crate::shared::subtance_type::SubstanceType;
use ndarray::{Array2, Zip};
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;
use serde::Serialize;
use tracing::instrument;

const ENLARGE_FACTOR: usize = 255;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct SubstanceDistribution {
    substance_type: SubstanceType,
    distribution: Array2<HexUnit>,
    noise_params: NoiseParams,
}

impl SubstanceDistribution {
    #[instrument(skip_all)]
    pub fn new(
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

    pub fn substance_type(&self) -> &SubstanceType {
        &self.substance_type
    }

    pub fn distribution(&self) -> &Array2<HexUnit> {
        &self.distribution
    }

    pub fn noise_params(&self) -> &NoiseParams {
        &self.noise_params
    }

    pub fn diffuse(&mut self) {}
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
