use crate::environment::{layer::Layer, resource_distribution::ResourceDistribution};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Landscape {
    pub layer: Layer,
    pub resource_distributions: HashSet<ResourceDistribution>,
}

impl Landscape {
    pub fn new(layer: Layer) -> Self {
        Self {
            layer,
            resource_distributions: HashSet::new(),
        }
    }

    pub fn add_resource_distribution(&mut self, resource_distribution: ResourceDistribution) {
        // 检查集合中是否已存在相同的 `resource_type`
        let exists = self
            .resource_distributions
            .par_iter()
            .any(|rd| rd.resource_type == resource_distribution.resource_type);

        // 如果不存在，则插入新的 `resource_distribution`
        if !exists {
            tracing::debug!(
                "成功插入新的 `resource_distribution`: {:?}",
                &resource_distribution
            );
            self.resource_distributions.insert(resource_distribution);
        } else {
            // 在 `info` 日志中包含 `resource_type` 的值
            tracing::info!(
                "已存在相同的 `resource_type`: {:?}，无法插入新的 `resource_distribution`",
                resource_distribution.resource_type
            );
        }
    }
}
