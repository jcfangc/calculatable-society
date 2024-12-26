use crate::environment::subtance_distribution::SubstanceDistribution;
use crate::game_context::GameContext;
use crate::shared::property::Property;
use ndarray::{parallel::prelude::*, Array2};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Potential {
    potential_distribution: Array2<f64>,
}

impl Potential {
    /// 创建新的 `Potential`
    pub(crate) fn new(map_size: (usize, usize)) -> Self {
        Self {
            potential_distribution: Array2::<f64>::zeros(map_size),
        }
    }

    /// 获取当前的势能场强分布
    pub(crate) fn distribution(&self) -> &Array2<f64> {
        &self.potential_distribution
    }

    /// 更新势能场强分布
    pub(crate) fn update(
        &mut self,
        subtance_distributions: &HashSet<SubstanceDistribution>,
        map_size: (usize, usize),
    ) {
        self.potential_distribution =
            self.calculate_potential_distribution(subtance_distributions, map_size);
    }

    /// 计算势能场强分布
    ///
    /// ### 参数
    /// - `subtance_distributions`: `&HashSet<SubstanceDistribution>`，物质分布的集合。
    ///   - 每个 `SubstanceDistribution` 表示一种物质的分布情况。
    ///   - 集合中的元素无序，但每种物质的类型是唯一的（通过 `HashSet` 确保）。
    /// - `map_size`: `(usize, usize)`，地图的大小。
    ///   - 用于定义返回势能场强分布的数组形状。
    ///
    /// ### 返回值
    /// 返回整个地图上的势能场强分布 `Array2<f64>`。
    /// - 返回的二维数组形状为 `map_size`。
    /// - 每个单元格的值表示该位置的势能场强值。
    fn calculate_potential_distribution(
        &self,
        subtance_distributions: &HashSet<SubstanceDistribution>,
        map_size: (usize, usize),
    ) -> Array2<f64> {
        subtance_distributions
            .par_iter()
            .map(|substance_dist| self.calculate_single_potential_dist(substance_dist, map_size))
            .reduce(|| Array2::<f64>::zeros(map_size), |acc, dist| acc + dist)
    }

    /// 计算单个物质分布的势能场强分布
    ///
    /// ### 参数
    /// - `substance_dist`: `SubstanceDistribution`，物质分布
    /// - `map_size`: `(usize, usize)`，地图的大小，用于定义势能场强分布的数组形状
    ///
    /// ### 返回值
    /// 返回势能场强分布 `Array2<f64>`
    fn calculate_single_potential_dist(
        &self,
        substance_dist: &SubstanceDistribution,
        map_size: (usize, usize),
    ) -> Array2<f64> {
        // 获取本物质分布所代表物质的属性：摩尔质量
        let molar_mass =
            Property::calculate_property(Property::MolarMass, substance_dist.substance_type());

        // 获取本物质分布所代表物质的属性：密度（密度非零）
        let density =
            Property::calculate_property(Property::Density, substance_dist.substance_type());

        // 用摩尔质量除以密度，得到一摩尔物质的体积（高度）
        let molar_height = molar_mass / density;

        // 获取重力常数
        let gravity_const = GameContext::get_gravity_const();

        // 将物质分布的每个网格单元的摩尔量乘以一摩尔物质的体积，得到势能场强分布
        Array2::from_shape_vec(
            map_size, // 将分布结果按照地图大小转换为二维数组
            substance_dist
                .distribution()
                .par_iter() // 并行遍历物质分布中的单元
                .map(|hex_unit| hex_unit.mole() as f64 * molar_height * gravity_const) // 计算单元势能场强
                .collect::<Vec<f64>>(), // 收集结果为向量
        )
        .expect("未能从向量创建势能场强分布") // 确保向量长度与地图形状匹配
    }
}
