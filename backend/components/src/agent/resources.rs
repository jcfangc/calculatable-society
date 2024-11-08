use crate::agent::resource_amount::ResourceAmount;
use crate::shared::property::Property;
use crate::shared::subtance_type::SubtanceType;
use futures::future::join_all;
use std::collections::HashMap;
use std::fmt;
use tokio::task;

/// `Resources` 结构体，用于管理多个 `Resource` 对象
#[derive(Debug)]
pub struct Resources {
    // 使用 HashMap 管理资源类型到资源的映射
    resources: HashMap<SubtanceType, ResourceAmount>,
}

impl Resources {
    /// 构造函数，创建一个 `Resources` 实例
    ///
    /// ### 参数
    /// - `resources`: 一个可选的初始资源类型系数和资源数量的键值对集合
    ///
    /// ### 返回值
    /// 返回一个新的 `Resources` 实例
    pub fn new(resources: Option<HashMap<SubtanceType, ResourceAmount>>) -> Self {
        Resources {
            resources: resources.unwrap_or_default(),
        }
    }
    /// 添加或更新资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要设置的资源数量
    pub fn set(&mut self, resource_type: SubtanceType, amount: ResourceAmount) {
        self.resources.insert(resource_type, amount);
    }

    /// 获取特定资源类型的资源数量
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    ///
    /// ### 返回值
    /// 返回一个 `Option<&ResourceAmount>`，如果存在则返回对应的资源引用，否则返回 `None`
    pub fn get(&self, resource_type: &SubtanceType) -> Option<&ResourceAmount> {
        self.resources.get(resource_type)
    }

    /// 删除特定资源类型的资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    pub fn remove(&mut self, resource_type: &SubtanceType) {
        self.resources.remove(resource_type);
    }

    /// 添加资源
    ///
    /// 如果资源已经存在，则增加数量；否则，添加新的资源。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要添加的资源数量
    pub fn add(&mut self, resource_type: SubtanceType, amount: ResourceAmount) {
        if let Some(existing_resource) = self.resources.get_mut(&resource_type) {
            *existing_resource += amount;
        } else {
            self.set(resource_type, amount);
        }
    }

    /// 减少资源
    ///
    /// 如果资源数量大于减少的数量，则减少数量；否则，删除资源。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要减少的资源数量
    pub fn minus(&mut self, resource_type: SubtanceType, amount: ResourceAmount) {
        if let Some(existing_resource) = self.resources.get_mut(&resource_type) {
            if *existing_resource > amount {
                *existing_resource -= amount;
            } else {
                self.remove(&resource_type);
            }
        }
    }

    /// 列出所有资源
    ///
    /// ### 返回值
    /// 返回一个包含所有资源的向量
    pub fn to_list(&self) -> Vec<(&SubtanceType, &ResourceAmount)> {
        self.resources.iter().collect()
    }

    /// 计算单个 `ResourceTypeCoefficient` 的属性值
    pub async fn get_properties(rtc: &SubtanceType) -> HashMap<Property, f64> {
        if let Ok(properties) = get_properties_by_numerator_and_dominator(
            *rtc.subtance_type.numer() as i32,
            *rtc.subtance_type.denom() as i32,
        )
        .await
        {
            return properties;
        }

        // 如果数据库没有匹配项，则进行计算
        let property_params = Property::to_map();

        let property_value_entries: HashMap<_, _> = property_params
            .into_iter() // 将 HashMap 转换为并行迭代器
            .map(|(property, property_const)| {
                let value = property_const.calculate(rtc);
                (*property, value)
            })
            .collect();

        property_value_entries
    }

    /// 计算所有资源的属性值
    pub async fn get_all_properties(&self) -> HashMap<SubtanceType, HashMap<Property, f64>> {
        // 仅在 `all_properties` 中使用 `spawn_blocking`，并移除 `properties_sync` 中的 `spawn_blocking`
        let futures: Vec<_> = self
            .resources
            .keys()
            .cloned()
            .map(|resource_type| {
                task::spawn_blocking(move || {
                    // 在阻塞线程池中同步计算属性
                    let properties = tokio::runtime::Handle::current()
                        .block_on(Resources::get_properties(&resource_type));
                    (resource_type, properties)
                })
            })
            .collect();

        let results = join_all(futures).await;

        results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }
}

// 为 Resources 实现 Default trait，使其可默认初始化为空映射
impl Default for Resources {
    fn default() -> Self {
        Resources {
            resources: HashMap::new(),
        }
    }
}

impl fmt::Display for Resources {
    /// 格式化 `Resources` 为字符串
    ///
    /// 将资源集合格式化为字符串，每个资源条目分行显示
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let resources: Vec<String> = self
            .resources
            .iter()
            .map(|(key, amount)| format!("{:?}: {:?}", key, amount))
            .collect();
        write!(f, "{}", resources.join("\n"))
    }
}
