//! 噪声地图模块
//!
//! 该模块提供了生成和操作噪声地图的功能，包括噪声数据的生成、统计信息的计算等。

use crate::environment::noise_map::noise_map_params::NoiseMapParams;
use crate::environment::noise_map::noise_map_stats::NoiseMapStatistics;
use crate::environment::noise_map::noise_map_type::NoiseMapType;
use ndarray::{Array2, Zip};
use noise::{NoiseFn, OpenSimplex};
use rand::Rng;
use serde::Serialize;
use std::fmt;

/// 噪声地图结构体。
///
/// `NoiseMap` 包含了生成的噪声数据和用于生成该数据的参数。
///
/// ### 类型参数
///
/// - `T`: 噪声数据的类型，必须实现 `NoiseMapType` 特质。
#[derive(Debug, Clone, Serialize)]
pub struct NoiseMap<T>
where
    T: NoiseMapType,
{
    /// 噪声数据，存储为二维数组。
    pub data: Array2<T>,
    /// 生成噪声数据时使用的参数。
    pub params: NoiseMapParams,
}

impl<T> NoiseMap<T>
where
    T: NoiseMapType,
{
    /// 生成一个新的噪声地图。
    ///
    /// ### 参数
    ///
    /// - `params`: 用于生成噪声地图的参数。
    ///
    /// ### 返回值
    ///
    /// 返回一个包含生成的噪声数据和参数的 `NoiseMap` 实例。
    ///
    /// ### 示例
    ///
    /// ```rust
    /// let params = NoiseMapParams::default();
    /// let noise_map = NoiseMap::<f32>::generate(params);
    /// ```
    pub fn generate(params: NoiseMapParams) -> Self {
        // 提取或生成随机种子和 scale 值
        let seed = params
            .seed
            .unwrap_or_else(|| rand::thread_rng().gen::<u32>());
        let simplex = OpenSimplex::new(seed);
        let scale = params
            .scale
            .unwrap_or_else(|| rand::thread_rng().gen_range(1.0..10.0));
        let frequency = scale * 2.0 * std::f64::consts::PI;

        // 使用默认维度
        let column_num = params.column_num.unwrap_or(256);
        let row_num = params.row_num.unwrap_or(256);

        // 创建噪声矩阵
        let mut data = Array2::<T>::zeros((row_num, column_num));
        Zip::indexed(&mut data).par_for_each(|(row_index, col_index), value| {
            let normalized_colidx = col_index as f64 / column_num as f64;
            let normalized_rowidx = row_index as f64 / row_num as f64;

            // 生成四维环绕噪声
            let s = (normalized_colidx * frequency).sin();
            let c = (normalized_colidx * frequency).cos();
            let t = (normalized_rowidx * frequency).sin();
            let u = (normalized_rowidx * frequency).cos();

            let noise_value = simplex.get([s, c, t, u]) as f32;
            *value = T::from_f32(noise_value).expect("从 f32 转换失败");
        });

        // 创建 NoiseMapParams 结构体
        let params = NoiseMapParams {
            row_num: Some(row_num),
            column_num: Some(column_num),
            seed: Some(seed),
            scale: Some(scale),
        };

        // 返回包含噪声数据和参数的 NoiseMap
        NoiseMap { data, params }
    }

    /// 获取噪声地图的统计信息。
    ///
    /// ### 返回值
    ///
    /// 返回一个 `NoiseMapStatistics` 实例，可用于计算和显示统计信息。
    pub fn statistics(&self) -> NoiseMapStatistics<T> {
        NoiseMapStatistics::new(&self.data)
    }

    /// 获取噪声数据的引用。
    ///
    /// ### 返回值
    ///
    /// 返回对内部噪声数据的引用。
    pub fn get_data(&self) -> &Array2<T> {
        &self.data
    }

    /// 获取噪声地图的初始化参数。
    ///
    /// ### 返回值
    ///
    /// 返回对 `NoiseMapParams` 的引用，包含用于生成噪声地图的初始化参数。
    pub fn get_params(&self) -> &NoiseMapParams {
        &self.params
    }
}

impl<T> fmt::Display for NoiseMap<T>
where
    T: NoiseMapType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(e) => write!(f, "序列化 NoiseMap 到 JSON 时出错: {}", e),
        }
    }
}

impl<T> Default for NoiseMap<T>
where
    T: NoiseMapType,
{
    /// 提供 `Default` 实现，生成默认的噪声地图。
    ///
    /// ### 返回值
    ///
    /// 返回使用默认参数生成的 `NoiseMap` 实例。
    fn default() -> Self {
        NoiseMap::generate(NoiseMapParams::default())
    }
}
