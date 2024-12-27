use crate::_commute::t_from_dto::FromDTO;
use crate::agent::resource_amount::ResourceAmount;
use crate::shared::property::Property;
use crate::shared::subtance_type::SubstanceType;
use back_core::repository::game_related::shared_related::property_related::get_properties_by_numerator_and_denominator_stream;
use futures::future::join_all;
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::fmt;
use tokio::task;

/// `Resources` 结构体，用于管理多个 `Resource` 对象
#[derive(Debug)]
pub(crate) struct Resources {
    // 使用 HashMap 管理资源类型到资源的映射
    resources: HashMap<SubstanceType, ResourceAmount>,
}

impl Resources {
    /// 构造函数，创建一个 `Resources` 实例
    ///
    /// ### 参数
    /// - `resources`: 一个可选的初始资源类型系数和资源数量的键值对集合
    ///
    /// ### 返回值
    /// 返回一个新的 `Resources` 实例
    pub(crate) fn new(resources: Option<HashMap<SubstanceType, ResourceAmount>>) -> Self {
        Resources {
            resources: resources.unwrap_or_default(),
        }
    }
    /// 添加或更新资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要设置的资源数量
    pub(crate) fn set(&mut self, resource_type: SubstanceType, amount: ResourceAmount) {
        self.resources.insert(resource_type, amount);
    }

    /// 获取特定资源类型的资源数量
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    ///
    /// ### 返回值
    /// 返回一个 `Option<&ResourceAmount>`，如果存在则返回对应的资源引用，否则返回 `None`
    pub(crate) fn get(&self, resource_type: &SubstanceType) -> Option<&ResourceAmount> {
        self.resources.get(resource_type)
    }

    /// 删除特定资源类型的资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    pub(crate) fn remove(&mut self, resource_type: &SubstanceType) {
        self.resources.remove(resource_type);
    }

    /// 添加资源
    ///
    /// 如果资源已经存在，则增加数量；否则，添加新的资源。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要添加的资源数量
    pub(crate) fn add(&mut self, resource_type: SubstanceType, amount: ResourceAmount) {
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
    pub(crate) fn minus(&mut self, resource_type: SubstanceType, amount: ResourceAmount) {
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
    pub(crate) fn to_list(&self) -> Vec<(&SubstanceType, &ResourceAmount)> {
        self.resources.iter().collect()
    }

    pub(crate) async fn get_properties(subtance_type: &SubstanceType) -> HashMap<Property, f64> {
        let mut property_value_entries = HashMap::new();

        // 创建异步流，获取属性数据
        let mut properties_stream = get_properties_by_numerator_and_denominator_stream(
            *subtance_type.ratio.numer() as i32,
            *subtance_type.ratio.denom() as i32,
            None,
        )
        .await;

        // 消费异步流，累积结果到 HashMap 中
        while let Some(batch_result) = properties_stream.next().await {
            match batch_result {
                Ok(batch) => {
                    for (property_params, property_value) in batch {
                        // 将 DTOPropertyParams 转换为 Property 类型
                        let property = Property::from_dto(property_params);
                        property_value_entries.insert(property, property_value);
                    }
                }
                Err(e) => {
                    tracing::error!("从数据库获取属性时出错: {:?}", e);
                }
            }
        }

        // 如果数据库没有匹配项，进行计算填充数据
        if property_value_entries.is_empty() {
            let property_params = Property::to_map();
            property_value_entries = property_params
                .into_iter()
                .map(|(property, property_const)| {
                    let value = property_const.calculate(subtance_type);
                    (*property, value)
                })
                .collect();
        }

        property_value_entries
    }

    /// 计算所有资源的属性值
    pub(crate) async fn get_all_properties(
        &self,
    ) -> HashMap<SubstanceType, HashMap<Property, f64>> {
        // 仅在 `all_properties` 中使用 `spawn_blocking`，并移除 `properties_sync` 中的 `spawn_blocking`
        let futures: Vec<_> = self
            .resources
            .keys()
            .cloned()
            .map(|subtance_type| {
                task::spawn_blocking(move || {
                    // 在阻塞线程池中同步计算属性
                    let properties = tokio::runtime::Handle::current()
                        .block_on(Resources::get_properties(&subtance_type));
                    (subtance_type, properties)
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
