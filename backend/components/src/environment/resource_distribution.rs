use crate::environment::layer::Layer;
use crate::environment::noise_params::NoiseParams;
use crate::environment::t_noise_generatable::NoiseGeneratable;
use crate::environment::t_statistical::Statistical;
use crate::shared::subtance_type::SubtanceType;
use ndarray::Array2;
use ndarray::Zip;
use noise::{NoiseFn, OpenSimplex};
use rayon::prelude::*;
use serde::Serialize;
use tracing::instrument;

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
        layer: Layer,
        noise_params: Option<NoiseParams>,
    ) -> Self {
        let noise_params = noise_params.unwrap_or_default(); // 使用 `NoiseParams` 的默认值
        let distribution = Array2::zeros((layer.height, layer.width));

        Self {
            resource_type,
            distribution,
            noise_params,
        }
    }
}

impl NoiseGeneratable for SubtanceDistribution {
    #[instrument(skip_all)]
    fn generate_simplex_noise(&mut self) {
        let simplex = OpenSimplex::new(self.noise_params.seed);
        let frequency = self.noise_params.scale * 2.0 * std::f64::consts::PI;

        let width = self.distribution.shape()[1];
        let height = self.distribution.shape()[0];

        // 使用并行的 Zip::indexed 和 par_for_each 处理矩阵中的每个元素
        Zip::indexed(&mut self.distribution).par_for_each(|(row_index, col_index), value| {
            // 计算归一化的行和列索引
            let normalized_colidx = col_index as f64 / width as f64;
            let normalized_rowidx = row_index as f64 / height as f64;

            // 生成四维环绕噪声
            let s = (normalized_colidx * frequency).sin();
            let c = (normalized_colidx * frequency).cos();
            let t = (normalized_rowidx * frequency).sin();
            let u = (normalized_rowidx * frequency).cos();

            // 生成噪声值并归一化到 0-255
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
