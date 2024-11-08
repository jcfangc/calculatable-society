use serde::Serialize;
use std::fmt;

const DEFAULT_WIDTH: usize = 255;
const DEFAULT_HEIGHT: usize = DEFAULT_WIDTH;

/// 地图参数结构体
#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub width: usize,
    pub height: usize,
}

impl Layer {
    pub fn new(width: Option<usize>, height: Option<usize>) -> Self {
        Self {
            width: width.unwrap_or(Self::default_width()),
            height: height.unwrap_or(Self::default_height()),
        }
    }

    fn default_width() -> usize {
        DEFAULT_WIDTH // 默认宽度
    }

    fn default_height() -> usize {
        DEFAULT_HEIGHT // 默认高度
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            height: Self::default_height(),
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => write!(f, "Layer({}, {})", self.width, self.height),
        }
    }
}
