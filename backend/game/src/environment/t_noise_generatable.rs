/// 生成特质，用于生成噪声地形
pub(crate) trait NoiseGeneratable {
    fn generate_simplex_noise(&mut self);
}
