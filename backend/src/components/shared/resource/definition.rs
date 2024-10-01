use crate::components::shared::resource::property::definition::PROPERTIES;
use crate::components::shared::resource::types::{ResourceAmount, ResourceTypeCoefficient};
use std::collections::HashMap;
use std::fmt;

/// `Resource` 结构体，表示单个资源类型及其数量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    pub resource_type: ResourceTypeCoefficient,
    pub amount: ResourceAmount,
}

impl Resource {
    /// 构造函数，创建一个新的 `Resource` 实例
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 资源数量
    ///
    /// # 返回值
    /// 返回一个新的 `Resource` 实例
    pub fn new(resource_type: ResourceTypeCoefficient, amount: ResourceAmount) -> Self {
        Resource {
            resource_type,
            amount,
        }
    }

    /// 获取资源的属性
    ///
    /// # 参数
    /// - `filter`: 可选的属性过滤器
    ///
    /// # 返回值
    /// 返回一个包含属性名称和值的哈希表
    ///
    /// # 示例
    /// ```
    /// let resource = Resource::new(ResourceTypeCoefficient::new(1, 1), ResourceAmount::new(10).unwrap());
    /// let properties = resource.get_properties(None);
    /// ```
    ///
    /// # 备注
    /// 如果 `filter` 存在，使用过滤后的属性列表，否则使用 `PROPERTIES` 中的所有属性
    pub fn get_properties(&self, filter: Option<Vec<String>>) -> HashMap<String, f64> {
        let mut properties: HashMap<String, f64> = HashMap::new();

        // 如果 `filter` 存在，使用过滤后的属性列表，否则使用 `PROPERTIES` 中的所有属性
        let property_names: Vec<&str> = match filter {
            Some(ref filter_list) => PROPERTIES
                .keys()
                .filter(|key| filter_list.contains(&key.to_string()))
                .copied()
                .collect(),
            None => PROPERTIES.keys().copied().collect(),
        };

        for property_name in property_names {
            if let Some(property_const) = PROPERTIES.get(property_name) {
                properties.insert(
                    property_name.to_string(),
                    property_const.calculate(&self.resource_type),
                );
            }
        }

        return properties;
    }
}

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
