use crate::environment::noise_map::noise_map_type::NoiseMapType;
use ndarray::Array2;
use rayon::prelude::*;
use serde::Serialize;

/// 噪声地图统计信息结构体。
///
/// 提供对噪声数据的统计计算，例如最小值、最大值、平均值和方差。
#[derive(Debug, Serialize)]
pub struct NoiseMapStatistics<'a, T>
where
    T: NoiseMapType,
{
    /// 引用噪声数据的二维数组。
    data: &'a Array2<T>,
}

impl<'a, T> NoiseMapStatistics<'a, T>
where
    T: NoiseMapType,
{
    /// 创建一个新的 `NoiseMapStatistics` 实例。
    ///
    /// ### 参数
    ///
    /// - `data`: 引用噪声数据的二维数组。
    ///
    /// ### 返回值
    ///
    /// 返回一个新的 `NoiseMapStatistics` 实例。
    pub fn new(data: &'a Array2<T>) -> Self {
        NoiseMapStatistics { data }
    }

    /// 计算噪声数据的最小值。
    pub fn min(&self) -> T {
        self.data
            .par_iter()
            .cloned()
            .reduce(|| T::infinity(), T::min)
    }

    /// 计算噪声数据的最大值。
    pub fn max(&self) -> T {
        self.data
            .par_iter()
            .cloned()
            .reduce(|| T::neg_infinity(), T::max)
    }

    /// 计算噪声数据的平均值。
    pub fn mean(&self) -> T {
        let sum: T = self.data.par_iter().cloned().sum();
        let count = T::from_usize(self.data.len()).expect("从 usize 转换失败");
        sum / count
    }

    /// 计算噪声数据的方差。
    pub fn variance(&self) -> T {
        let mean = self.mean();
        let sum_of_squares: T = self
            .data
            .par_iter()
            .cloned()
            .map(|value| {
                let diff = value - mean;
                diff * diff
            })
            .sum();

        let count = T::from_usize(self.data.len() - 1).expect("从 usize 转换失败");
        sum_of_squares / count
    }

    /// 返回统计信息的字符串表示。
    pub fn display(&self) -> String {
        format!(
            "统计信息:\n维度: {:?}, 最小值: {}, 最大值: {}, 平均值: {}, 方差: {}",
            self.data.dim(),
            self.min(),
            self.max(),
            self.mean(),
            self.variance()
        )
    }
}
