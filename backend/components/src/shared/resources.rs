use crate::shared::resource::types::resource_type_coefficient::ResourceTypeCoefficient;
use crate::shared::resource::Resource;
use std::collections::HashMap;
use std::fmt;

/// `Resources` 结构体，用于管理多个 `Resource` 对象
#[derive(Debug)]
pub struct Resources {
    // 使用 HashMap 管理资源类型到资源的映射
    resources: HashMap<ResourceTypeCoefficient, Resource>,
}

impl Resources {
    /// 构造函数，创建一个 `Resources` 实例
    ///
    /// # 参数
    /// - `resources`: 一个可选的初始资源集合
    ///
    /// # 返回值
    /// 返回一个新的 `Resources` 实例
    pub fn new(resources: Option<Vec<Resource>>) -> Self {
        let mut resource_map = HashMap::new();
        if let Some(resource_list) = resources {
            for resource in resource_list {
                resource_map.insert(resource.resource_type, resource);
            }
        }
        return Resources {
            resources: resource_map,
        };
    }

    /// 添加或更新资源
    ///
    /// # 参数
    /// - `resource`: 要添加或更新的 `Resource` 对象
    pub fn set(&mut self, resource: Resource) {
        self.resources.insert(resource.resource_type, resource);
    }

    /// 获取特定资源类型的资源
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    ///
    /// # 返回值
    /// 返回一个 `Option<&Resource>`，如果存在则返回对应的资源引用，否则返回 `None`
    pub fn get(&self, resource_type: &ResourceTypeCoefficient) -> Option<&Resource> {
        self.resources.get(resource_type)
    }

    /// 删除特定资源类型的资源
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    pub fn remove(&mut self, resource_type: &ResourceTypeCoefficient) {
        self.resources.remove(resource_type);
    }

    /// 添加资源
    ///
    /// 如果资源已经存在，则增加数量；否则，添加新的资源。
    ///
    /// # 参数
    ///
    /// - `resource`: 要添加的资源
    pub fn add(&mut self, resource: Resource) {
        if let Some(existing_resource) = self.resources.get_mut(&resource.resource_type) {
            existing_resource.amount += resource.amount;
        } else {
            self.set(resource);
        }
    }

    /// 减少资源
    ///
    /// 如果资源数量大于减少的数量，则减少数量；否则，删除资源。
    ///
    /// # 参数
    ///
    /// - `resource`: 要减少的资源
    pub fn minus(&mut self, resource: Resource) {
        if let Some(existing_resource) = self.resources.get_mut(&resource.resource_type) {
            if existing_resource.amount > resource.amount {
                existing_resource.amount -= resource.amount;
            } else {
                self.remove(&resource.resource_type);
            }
        }
    }

    /// 列出所有资源
    ///
    /// # 返回值
    /// 返回一个包含所有资源的向量
    pub fn list(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }
}

impl fmt::Display for Resources {
    /// 格式化 `Resources` 为字符串
    ///
    /// 将资源集合格式化为字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let resources_str = self
            .resources
            .values()
            .map(|res| format!("{:?}", res))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", resources_str)
    }
}
