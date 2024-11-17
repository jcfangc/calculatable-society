use serde::Serialize;
use std::fmt;

const DEFAULT_WIDTH: usize = 255; // 默认宽度常量
const DEFAULT_HEIGHT: usize = DEFAULT_WIDTH; // 默认高度常量，设为与宽度相同

/// 地图参数结构体
#[derive(Debug, Clone, Serialize, Copy)]
pub struct MapSize {
    height: usize, // 地图高度
    width: usize,  // 地图宽度
}

impl MapSize {
    /// 使用可选的宽度和高度创建一个新的 `MapSize` 实例。
    /// 如果宽度或高度未提供，则使用默认值。
    pub fn new(width: Option<usize>, height: Option<usize>) -> Self {
        Self {
            height: height.unwrap_or(Self::default_height()), // 如果未提供高度，使用默认高度
            width: width.unwrap_or(Self::default_width()),    // 如果未提供宽度，使用默认宽度
        }
    }

    /// 获取默认高度
    fn default_height() -> usize {
        DEFAULT_HEIGHT // 返回默认高度
    }

    /// 获取默认宽度
    fn default_width() -> usize {
        DEFAULT_WIDTH // 返回默认宽度
    }

    /// 获取 `MapSize` 的高度
    pub fn height(&self) -> usize {
        self.height
    }

    /// 获取 `MapSize` 的宽度
    pub fn width(&self) -> usize {
        self.width
    }

    /// 将 `MapSize` 转换为元组形式 (height, width)
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

impl Default for MapSize {
    /// 实现 `Default` trait，返回使用默认高度和宽度的 `MapSize` 实例
    fn default() -> Self {
        Self {
            height: Self::default_height(),
            width: Self::default_width(),
        }
    }
}

impl fmt::Display for MapSize {
    /// 实现 `Display` trait，将 `MapSize` 格式化为 JSON 字符串
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            // 尝试将结构体序列化为 JSON 字符串
            Ok(json) => write!(f, "{}", json), // 如果成功，写入 JSON 字符串
            Err(_) => write!(f, "MapSize({}, {})", self.height, self.width), // 如果失败，使用备用格式
        }
    }
}
