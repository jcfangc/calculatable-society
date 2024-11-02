use crate::environment::noise_map::{
    main::NoiseMap, noise_map_params::NoiseMapParams, noise_map_stats::NoiseMapStatistics,
    noise_map_type::NoiseMapType,
};
use ndarray::Array2;
use std::fmt;

/// `LayerTrait` 定义了一个生成和管理噪声地图的通用接口。
/// 实现此 trait 的类型可以生成基于噪声的地图数据，并提供统计信息和初始化参数的显示。
pub trait LayerTrait: fmt::Display + fmt::Debug + Default {
    /// `LayerValueType` 是噪声地图中的元素类型，必须实现 `NoiseMapType`。
    type LayerValueType: NoiseMapType;

    /// 使用 `NoiseMapParams` 初始化并生成噪声地图。
    ///
    /// ### 参数
    /// - `params`：`NoiseMapParams` 类型的参数，包含地图生成的行数、列数、种子、缩放比例等信息。
    ///
    /// ### 返回值
    /// 返回一个包含生成数据和初始化参数的 `NoiseMap` 实例。
    ///
    /// ### 示例
    /// ```
    /// let params = NoiseMapParams::new(Some(256), Some(256), Some(42), Some(1.5));
    /// let noise_map = LayerTrait::generate_noise_map(params);
    /// ```
    fn generate_noise_map(params: NoiseMapParams) -> NoiseMap<Self::LayerValueType>
    where
        Self: Sized,
    {
        NoiseMap::generate(params)
    }
    /// 获取噪声地图数据，返回地图数据的引用。
    ///
    /// ### 返回值
    /// 返回 `Array2<Self::LayerValueType>` 类型的引用，其中包含了噪声地图的实际数据。
    ///
    /// ### 示例
    /// ```
    /// let data = layer.get_data();
    /// ```
    fn get_data(&self) -> &Array2<Self::LayerValueType>;
    /// 获取初始化参数，返回初始化参数的字符串格式。
    ///
    /// ### 返回值
    /// 返回 `NoiseMapParams` 类型的引用，包含用于生成噪声地图的初始化参数。
    ///
    /// ### 示例
    /// ```
    /// let params = layer.get_params();
    /// println!("Params: {}", params);
    /// ```
    fn get_params(&self) -> &NoiseMapParams;
    /// 显示初始化参数，默认使用 `get_params` 方法的返回值。
    ///
    /// ### 返回值
    /// 返回包含初始化参数的字符串表示。
    ///
    /// ### 示例
    /// ```
    /// let init_params = layer.display_init_params();
    /// println!("{}", init_params);
    /// ```
    fn display_init_params(&self) -> String {
        format!("{:?}", self.get_params())
    }
    /// 获取噪声地图的统计信息，包括最小值、最大值、平均值和方差。
    ///
    /// ### 返回值
    /// 返回 `NoiseMapStatistics<Self::LayerValueType>` 实例，包含当前噪声地图的数据统计信息。
    ///
    /// ### 示例
    /// ```
    /// let stats = layer.statistics();
    /// println!("Min: {}, Max: {}", stats.min(), stats.max());
    /// ```
    fn statistics(&self) -> NoiseMapStatistics<Self::LayerValueType> {
        NoiseMapStatistics::new(self.get_data())
    }
    /// 显示统计信息，利用 `NoiseMapStatistics` 结构体计算并格式化输出。
    ///
    /// ### 返回值
    /// 返回包含统计信息的字符串格式。
    ///
    /// ### 示例
    /// ```
    /// let stats = layer.display_statistics();
    /// println!("{}", stats);
    /// ```
    fn display_statistics(&self) -> String {
        let stats = self.statistics();
        stats.display()
    }
}
