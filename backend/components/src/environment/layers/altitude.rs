use crate::environment::layers::layer_trait::LayerTrait;
use crate::environment::noise_map::{main::NoiseMap, noise_map_params::NoiseMapParams};
use ndarray::Array2;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug, Default)]
struct AltitudeLayer {
    map: NoiseMap<f32>,
}

impl LayerTrait for AltitudeLayer {
    type LayerValueType = f32;

    fn get_data(&self) -> &Array2<Self::LayerValueType> {
        &self.map.data
    }

    fn get_params(&self) -> &NoiseMapParams {
        &self.map.params
    }
}

impl Display for AltitudeLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 直接调用 self.map 的 Display 实现
        write!(f, "{}", self.map)
    }
}
