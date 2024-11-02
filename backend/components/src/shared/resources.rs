use crate::shared::property::repository::get_properties_by_numerator_and_dominator;
use crate::shared::property::Property;
use crate::shared::resource_amount::ResourceAmount;
use crate::shared::resource_type_coefficient::ResourceTypeCoefficient;
use futures::future::join_all;
use std::collections::HashMap;
use std::fmt;
use tokio::task;

/// `Resources` 结构体，用于管理多个 `Resource` 对象
#[derive(Debug)]
pub struct Resources {
    // 使用 HashMap 管理资源类型到资源的映射
    resources: HashMap<ResourceTypeCoefficient, ResourceAmount>,
}

impl Resources {
    /// 构造函数，创建一个 `Resources` 实例
    ///
    /// ### 参数
    /// - `resources`: 一个可选的初始资源类型系数和资源数量的键值对集合
    ///
    /// ### 返回值
    /// 返回一个新的 `Resources` 实例
    pub fn new(resources: Option<HashMap<ResourceTypeCoefficient, ResourceAmount>>) -> Self {
        Resources {
            resources: resources.unwrap_or_default(),
        }
    }
    /// 添加或更新资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要设置的资源数量
    pub fn set(&mut self, resource_type: ResourceTypeCoefficient, amount: ResourceAmount) {
        self.resources.insert(resource_type, amount);
    }

    /// 获取特定资源类型的资源数量
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    ///
    /// ### 返回值
    /// 返回一个 `Option<&ResourceAmount>`，如果存在则返回对应的资源引用，否则返回 `None`
    pub fn get(&self, resource_type: &ResourceTypeCoefficient) -> Option<&ResourceAmount> {
        self.resources.get(resource_type)
    }

    /// 删除特定资源类型的资源
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    pub fn remove(&mut self, resource_type: &ResourceTypeCoefficient) {
        self.resources.remove(resource_type);
    }

    /// 添加资源
    ///
    /// 如果资源已经存在，则增加数量；否则，添加新的资源。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要添加的资源数量
    pub fn add(&mut self, resource_type: ResourceTypeCoefficient, amount: ResourceAmount) {
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
    pub fn minus(&mut self, resource_type: ResourceTypeCoefficient, amount: ResourceAmount) {
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
    pub fn to_list(&self) -> Vec<(&ResourceTypeCoefficient, &ResourceAmount)> {
        self.resources.iter().collect()
    }

    /// 计算单个 `ResourceTypeCoefficient` 的属性值
    pub async fn get_properties(rtc: &ResourceTypeCoefficient) -> HashMap<Property, f64> {
        if let Ok(properties) = get_properties_by_numerator_and_dominator(
            *rtc.resource_type.numer() as i32,
            *rtc.resource_type.denom() as i32,
        )
        .await
        {
            return properties;
        }

        // 如果数据库没有匹配项，则进行计算
        let property_params = Property::to_map().await;

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
    pub async fn get_all_properties(
        &self,
    ) -> HashMap<ResourceTypeCoefficient, HashMap<Property, f64>> {
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
// resources 组件

// Route 模块: 定义路由，将 URL 映射到具体的处理方法
mod route {
    //Route 模块负责定义连接的路由规则，将 URL 映射到 Controller 中的处理方法。
    // 例如: Router::new().route("//resources", (handle_resources_))
}

// Controller 模块: 处理来自客户端的请求，负责连接的生命周期管理
mod controller {
    // Controller 负责处理连接的建立、消息收发以及关闭等生命周期操作。
    // 它接收来自客户端的消息，并将消息传递给 Service 进行业务处理。
    // 例如: async fn handle_resources_(socket: WebSocket) -> impl IntoResponse { ... }
}

// Service 模块: 负责处理消息的业务逻辑，调用 Repository 获取或更新数据
mod service {
    // Service 处理来自 Controller 的消息，执行具体的业务逻辑。
    // 例如: 处理收到的消息，更新数据库中的状态或发送响应消息。
    // 例如: async fn process_resources_message(message: String) -> Result<String, Error> { ... }
}

// Repository 模块: 负责与数据库的交互，执行数据的增删改查操作
mod repository {
    use super::model::ResourcesModel;
    use crate::shared::resource_amount::ResourceAmount;
    use crate::shared::resource_type_coefficient::ResourceTypeCoefficient;
    use context::db::context::DatabaseContext;
    use context::GLOBAL_APP_CONTEXT;
    use sqlx::Error;
    use uuid::Uuid;

    pub async fn get_resource_by_id_numerator_and_dominator(
        agent_id: Uuid,
        resource_numerator: i32,
        resource_dominator: i32,
    ) -> Result<(ResourceTypeCoefficient, ResourceAmount), Error> {
        if resource_dominator <= 0 || resource_numerator <= 0 {
            return Err(sqlx::Error::Protocol(
                "resource_numerator 和 resource_dominator 参数应该为正数".into(),
            ));
        }

        let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

        // 执行查询
        if let Some(resources_model) = sqlx::query_as::<_, ResourcesModel>(
            r#"
            SELECT 
                agent_id,
                numerator,
                denominator,
                allocatable,
                investment,
                debt
            FROM 
                resources
            WHERE 
                agent_id = $1 AND numerator = $2 AND denominator = $3
            "#,
        )
        .bind(agent_id)
        .bind(resource_numerator)
        .bind(resource_dominator)
        .fetch_optional(pool)
        .await?
        {
            if let Ok(resource_type_coefficient) = ResourceTypeCoefficient::new(
                resources_model.numerator as usize,
                resources_model.denominator as usize,
            ) {
                let resource_amount = ResourceAmount::new(
                    resources_model.allocatable as usize,
                    resources_model.investment as usize,
                    resources_model.debt as usize,
                );

                return Ok((resource_type_coefficient, resource_amount));
            }
        }

        // 如果没有找到对应的记录，返回一个自定义错误或选择返回一个默认值
        Err(sqlx::Error::RowNotFound)
    }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    use sqlx::FromRow;
    use uuid::Uuid;

    #[derive(FromRow)]
    pub struct ResourcesModel {
        pub agent_id: Uuid,
        pub numerator: i32,
        pub denominator: i32,
        pub allocatable: i32,
        pub investment: i32,
        pub debt: i32,
    }
}
