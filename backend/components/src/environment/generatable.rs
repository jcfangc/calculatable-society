use crate::environment::resource_distribution::ResourceDistribution;
use ndarray::Zip;
use noise::{NoiseFn, OpenSimplex};

const ENLARGE_FACTOR: usize = 255;

/// 生成特质，用于生成噪声地形
trait Generatable {
    fn generate_simplex_noise(&mut self);
}

impl Generatable for ResourceDistribution {
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
