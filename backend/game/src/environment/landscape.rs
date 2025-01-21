use crate::environment::potential::Potential;
use crate::environment::{map_size::MapSize, subtance_distribution::SubstanceDistribution};
use crate::game_context::GameContext;
use ndarray::parallel::prelude::*;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Landscape {
    map_size: MapSize,
    subtance_distributions: HashSet<SubstanceDistribution>,
    potential: Potential,
}

impl Landscape {
    pub(crate) fn new(map_size: MapSize) -> Self {
        // 更新全局上下文中的 map_size
        GameContext::update_global_map_size(map_size.clone());

        // 创建新的 Landscape
        Self {
            map_size,
            subtance_distributions: HashSet::new(),
            potential: Potential::new(map_size.as_tuple()),
        }
    }

    pub(crate) fn map_size(&self) -> &MapSize {
        &self.map_size
    }

    pub(crate) fn subtance_distributions(&self) -> &HashSet<SubstanceDistribution> {
        &self.subtance_distributions
    }

    pub(crate) fn potential(&self) -> &Potential {
        &self.potential
    }

    pub(crate) fn add_resource_distribution(
        &mut self,
        subtance_distribution: SubstanceDistribution,
    ) {
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

    pub(crate) fn update_potential_distribution(&mut self) {
        // 计算势能场强分布
        self.potential
            .update(&self.subtance_distributions, self.map_size.as_tuple());
    }
}

/// 关于扩散逻辑的集合
impl Landscape {
    pub(crate) fn diffuse(&mut self) {
        // 更新当前状态
        self.update_distributions(None);
    }

    /// 更新物质分布集合
    fn update_distributions(
        &mut self,
        updated_distributions: Option<HashSet<SubstanceDistribution>>,
    ) {
        if let Some(distributions) = updated_distributions {
            // 如果提供了更新的分布，直接使用
            self.subtance_distributions = distributions;
        } else {
            // 如果没有提供，调用 `calculate_diffusion` 进行计算
            self.subtance_distributions = self.calculate_diffusion();
        }
    }

    /// 计算扩散后的新物质分布状态
    fn calculate_diffusion(&self) -> HashSet<SubstanceDistribution> {
        self.subtance_distributions
            .par_iter()
            .map(|substance_dist| {
                let mut updated = substance_dist.clone();
                updated.diffuse(self.potential());
                updated
            })
            .collect() // Rayon 的并行收集可以直接构建 HashSet
    }
}
