use crate::environment::{map_size::MapSize, subtance_distribution::SubtanceDistribution};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Landscape {
    pub layer: MapSize,
    pub subtance_distributions: HashSet<SubtanceDistribution>,
}

impl Landscape {
    pub fn new(layer: MapSize) -> Self {
        Self {
            layer,
            subtance_distributions: HashSet::new(),
        }
    }

    pub fn add_resource_distribution(&mut self, subtance_distribution: SubtanceDistribution) {
        // 检查集合中是否已存在相同的 `resource_type`
        let exists = self
            .subtance_distributions
            .par_iter()
            .any(|rd| rd.resource_type == subtance_distribution.resource_type);

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
                subtance_distribution.resource_type
            );
        }
    }
}
