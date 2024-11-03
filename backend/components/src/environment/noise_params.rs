use rand::Rng;
use serde::Serialize;
use std::fmt;
use std::hash::{Hash, Hasher};

const DEFAULT_SCALE_RANGE: std::ops::Range<f64> = 1.0..10.0;

#[derive(Debug, Clone, Serialize)]
pub struct NoiseParams {
    pub seed: u32,
    pub scale: f64,
}

impl NoiseParams {
    pub fn new(seed: Option<u32>, scale: Option<f64>) -> Self {
        Self {
            seed: seed.unwrap_or_else(Self::default_seed),
            scale: scale.unwrap_or_else(Self::default_scale),
        }
    }

    pub fn default_seed() -> u32 {
        rand::thread_rng().gen::<u32>()
    }

    pub fn default_scale() -> f64 {
        rand::thread_rng().gen_range(DEFAULT_SCALE_RANGE)
    }
}

impl PartialEq for NoiseParams {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed && (self.scale - other.scale).abs() < f64::EPSILON
    }
}

impl Eq for NoiseParams {}

impl Hash for NoiseParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        // 将 f64 转换为整数来计算哈希，以便更稳定的哈希值
        (self.scale.to_bits()).hash(state);
    }
}

impl Default for NoiseParams {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl fmt::Display for NoiseParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(_) => write!(f, "NoiseParams(seed: {}, scale: {})", self.seed, self.scale),
        }
    }
}
