use crate::environment::potential::Potential;
use crate::environment::{map_size::MapSize, subtance_distribution::SubstanceDistribution};
use crate::shared::property::Property;
use ndarray::parallel::prelude::*;
use ndarray::{Array2, Zip};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Landscape {
    map_size: MapSize,
    subtance_distributions: HashSet<SubstanceDistribution>,
    potential: Potential,
}

impl Landscape {
    pub fn new(map_size: MapSize) -> Self {
        Self {
            map_size,
            subtance_distributions: HashSet::new(),
            potential: Potential::new(map_size.as_tuple()),
        }
    }

    pub fn map_size(&self) -> &MapSize {
        &self.map_size
    }

    pub fn subtance_distributions(&self) -> &HashSet<SubstanceDistribution> {
        &self.subtance_distributions
    }

    pub fn potential(&self) -> &Array2<f64> {
        &self.potential.distribution()
    }

    pub fn add_resource_distribution(&mut self, subtance_distribution: SubstanceDistribution) {
        // 检查集合中是否已存在相同的 `resource_type`
        let exists = self
            .subtance_distributions
            .par_iter()
            .any(|sd| sd.substance_type() == subtance_distribution.substance_type());

        // 如果不存在，则插入新的 `resource_distribution`
        if !exists {
            tracing::trace!(
                "成功插入新的 `resource_distribution`: {:?}",
                &subtance_distribution
            );
            self.subtance_distributions.insert(subtance_distribution);
        } else {
            // 在 `info` 日志中包含 `resource_type` 的值
            tracing::debug!(
                "已存在相同的 `resource_type`: {:?}，无法插入新的 `resource_distribution`",
                subtance_distribution.substance_type()
            );
        }
    }

    pub fn update_potential_distribution(&mut self) {
        // 计算势能分布
        self.potential
            .update(&self.subtance_distributions, self.map_size.as_tuple());
    }

    pub fn diffuse(&mut self) {
        // 计算扩散后的新状态
        let updated_distributions = self.calculate_diffusion();

        // 更新当前状态
        self.update_distributions(updated_distributions);
    }

    /// 计算扩散后的新物质分布状态
    fn calculate_diffusion(&self) -> Vec<SubstanceDistribution> {
        // 将 HashSet 转换为 Vec
        let substance_vec: Vec<SubstanceDistribution> =
            self.subtance_distributions.par_iter().cloned().collect();

        // 并行遍历物质分布并计算新状态
        substance_vec
            .into_par_iter()
            .map(|mut substance_dist| {
                // 调用物质分布层的扩散逻辑
                substance_dist.diffuse();
                substance_dist
            })
            .collect()
    }

    /// 更新物质分布集合
    fn update_distributions(&mut self, updated_distributions: Vec<SubstanceDistribution>) {
        // 清空当前 HashSet
        self.subtance_distributions.clear();

        // 将更新后的分布加入 HashSet
        self.subtance_distributions = updated_distributions.into_iter().collect();
    }
}
