use crate::environment::layer::Layer;
use crate::environment::noise_params::NoiseParams;
use crate::shared::resource_type::ResourceType;
use ndarray::Array2;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct ResourceDistribution {
    pub resource_type: ResourceType,
    pub distribution: Array2<usize>,
    pub noise_params: NoiseParams,
}

impl ResourceDistribution {
    pub fn new(
        resource_type: ResourceType,
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
