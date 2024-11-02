use serde::Serialize;
use std::fmt;

/// 噪声地图的参数结构体。
///
/// 包含生成噪声地图时所需的参数，例如行数、列数、随机种子和缩放因子。
#[derive(Debug, Clone, Serialize)]
pub struct NoiseMapParams {
    /// 噪声地图的行数。
    pub row_num: Option<usize>,
    /// 噪声地图的列数。
    pub column_num: Option<usize>,
    /// 随机种子，用于生成噪声。
    pub seed: Option<u32>,
    /// 缩放因子，影响噪声的频率。
    pub scale: Option<f64>,
}

impl NoiseMapParams {
    /// 创建一个新的 `NoiseMapParams` 实例。
    ///
    /// ### 参数
    ///
    /// - `row_num`: 行数。
    /// - `column_num`: 列数。
    /// - `seed`: 随机种子。
    /// - `scale`: 缩放因子。
    ///
    /// ### 返回值
    ///
    /// 返回一个新的 `NoiseMapParams` 实例。
    pub fn new(
        row_num: Option<usize>,
        column_num: Option<usize>,
        seed: Option<u32>,
        scale: Option<f64>,
    ) -> Self {
        NoiseMapParams {
            row_num,
            column_num,
            seed,
            scale,
        }
    }
}
impl fmt::Display for NoiseMapParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用 serde_json 将结构体序列化为 JSON 字符串
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(e) => write!(f, "将 NoiseMapParams 序列化为 JSON 时出错: {}", e),
        }
    }
}
impl Default for NoiseMapParams {
    /// 提供 `Default` 实现，返回默认参数。
    ///
    /// 默认行数和列数为 256，随机种子和缩放因子为 `None`。
    fn default() -> Self {
        NoiseMapParams {
            row_num: Some(256),
            column_num: Some(256),
            seed: None,
            scale: None,
        }
    }
}
