use crate::shared::property::repository::get_properties_by_numerator_and_dominator;
use crate::shared::property::Property;
use crate::shared::resources::types::resource_amount::ResourceAmount;
use crate::shared::resources::types::resource_type_coefficient::ResourceTypeCoefficient;
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
    /// # 参数
    /// - `resources`: 一个可选的初始资源类型系数和资源数量的键值对集合
    ///
    /// # 返回值
    /// 返回一个新的 `Resources` 实例
    pub fn new(resources: Option<HashMap<ResourceTypeCoefficient, ResourceAmount>>) -> Self {
        Resources {
            resources: resources.unwrap_or_default(),
        }
    }
    /// 添加或更新资源
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    /// - `amount`: 要设置的资源数量
    pub fn set(&mut self, resource_type: ResourceTypeCoefficient, amount: ResourceAmount) {
        self.resources.insert(resource_type, amount);
    }

    /// 获取特定资源类型的资源数量
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数
    ///
    /// # 返回值
    /// 返回一个 `Option<&ResourceAmount>`，如果存在则返回对应的资源引用，否则返回 `None`
    pub fn get(&self, resource_type: &ResourceTypeCoefficient) -> Option<&ResourceAmount> {
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
    /// # 参数
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
    /// # 返回值
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
mod repository {}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct ResourcesModel {
        pub agent_id: String,
        pub numerator: i32,
        pub denominator: i32,
        pub allocatable: i32,
        pub investment: i32,
        pub debt: i32,
    }
}

// Types 模块: 封装与组件相关的基础类型，便于全局使用
pub mod types {
    pub mod resource_amount {
        use std::fmt;
        use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
        use tracing::error;

        // 自定义 "资源数量" 类型并进行验证
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct ResourceAmount {
            allocatable: u32, // 表示当前可分配的资源量
            investment: u32,  // 累积投资总量
            debt: u32,        // 累积债务总量
        }

        impl ResourceAmount {
            /// 构造函数，创建 `ResourceAmount` 实例
            pub fn new(allocatable: u32, investment: u32, debt: u32) -> Self {
                ResourceAmount {
                    allocatable,
                    investment,
                    debt,
                }
            }

            /// 初始化 `ResourceAmount` 实例
            ///
            /// 创建一个初始的 `ResourceAmount` 实例，所有字段都为 0。
            pub fn init() -> Self {
                ResourceAmount {
                    allocatable: 0,
                    investment: 0,
                    debt: 0,
                }
            }

            /// 最终设置完成的方法，用于结束链式调用
            pub fn finalize(&mut self) {
                // 这里可以执行一些最终的操作，比如打印日志等
            }
        }

        impl AddAssign for ResourceAmount {
            fn add_assign(&mut self, other: Self) {
                // 将所有字段都进行累加操作
                self.allocatable += other.allocatable;
                self.investment += other.investment;
                self.debt += other.debt;
            }
        }

        impl SubAssign for ResourceAmount {
            /// 重载 `-=` 运算符，实现资源数量的减法
            fn sub_assign(&mut self, other: Self) {
                self.allocatable = self.allocatable.saturating_sub(other.allocatable);
                self.investment = self.investment.saturating_sub(other.investment);
                self.debt = self.debt.saturating_sub(other.debt);
            }
        }

        impl MulAssign for ResourceAmount {
            /// 重载 `*=` 运算符，实现资源数量的乘法
            fn mul_assign(&mut self, other: Self) {
                self.allocatable *= other.allocatable;
                self.investment *= other.investment;
                self.debt *= other.debt;
            }
        }

        impl DivAssign for ResourceAmount {
            /// 重载 `/=` 运算符，实现资源数量的除法
            fn div_assign(&mut self, other: Self) {
                if other.allocatable == 0 || other.investment == 0 || other.debt == 0 {
                    error!("除数不能为 0！");
                    return;
                }
                self.allocatable /= other.allocatable;
                self.investment /= other.investment;
                self.debt /= other.debt;
            }
        }

        // 定义 allocatable 字段的相关行为
        pub trait AllocatableOperation {
            fn get_allocatable(&self) -> u32;
            fn set_allocatable(&mut self, new_value: u32) -> &mut Self;
        }

        impl AllocatableOperation for ResourceAmount {
            fn get_allocatable(&self) -> u32 {
                self.allocatable
            }

            fn set_allocatable(&mut self, new_value: u32) -> &mut Self {
                self.allocatable = new_value;
                self
            }
        }

        // 定义 investment 字段的相关行为
        pub trait InvestmentOperation {
            fn get_investment(&self) -> u32;
            fn set_investment(&mut self, new_value: u32) -> &mut Self;
        }

        impl InvestmentOperation for ResourceAmount {
            fn get_investment(&self) -> u32 {
                self.investment
            }

            fn set_investment(&mut self, new_value: u32) -> &mut Self {
                self.investment = new_value;
                self
            }
        }

        // 定义 debt 字段的相关行为
        pub trait DebtOperation {
            fn get_debt(&self) -> u32;
            fn set_debt(&mut self, new_value: u32) -> &mut Self;
        }

        impl DebtOperation for ResourceAmount {
            fn get_debt(&self) -> u32 {
                self.debt
            }

            fn set_debt(&mut self, new_value: u32) -> &mut Self {
                self.debt = new_value;
                self
            }
        }

        impl fmt::Display for ResourceAmount {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "ResourceAmount {{ allocatable: {}, investment: {}, debt: {} }}",
                    self.allocatable, self.investment, self.debt
                )
            }
        }
    }
    pub mod resource_type_coefficient {
        use num::rational::Ratio;
        use std::fmt;
        use tracing::error;
        use validator::ValidationError;

        // 使用 const 来定义编译期常量上下界
        const LOWER_BOUND: Ratio<u32> = Ratio::new_raw(0, 1); // 下限 0
        const UPPER_BOUND: Ratio<u32> = Ratio::new_raw(2, 1); // 上限 2

        // 自定义 "资源角系数" 类型，封装 Ratio 并进行验证
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub struct ResourceTypeCoefficient {
            pub resource_type: Ratio<u32>, // 使用无符号整数，表示资源的种类及精密度
        }

        impl ResourceTypeCoefficient {
            /// 构造函数，创建 `ResourceTypeCoefficient` 实例
            ///
            /// 资源角系数在创建时必须经过验证，确保其在有效范围内。如果超出上下界，将返回验证错误。
            ///
            /// # 参数
            /// - `resource_type`: 资源的类型系数，使用 `Ratio<u32>` 表示。
            ///
            /// # 返回值
            /// 返回一个 `Result<Self, ValidationError>`，其中 `Self` 是构造成功的 `ResourceTypeCoefficient` 实例，`ValidationError` 则是当系数超出有效范围时返回的错误。
            ///
            /// # 错误
            /// 当 `resource_type` 小于 `LOWER_BOUND` 或大于 `UPPER_BOUND` 时，会返回 `ValidationError`。
            pub fn new(resource_type: Ratio<u32>) -> Result<Self, ValidationError> {
                if resource_type < LOWER_BOUND || resource_type > UPPER_BOUND {
                    error!("{} 是非法的资源种类（资源角系数）！", resource_type);
                    return Err(ValidationError::new("非法的资源种类（资源角系数）！"));
                }
                Ok(ResourceTypeCoefficient { resource_type })
            }

            /// 计算资源种类的精密度
            ///
            /// 根据 `Ratio<u32>` 的分母部分，计算资源种类的精度。返回分母部分的字符串长度，表示该种类的精密度。
            ///
            /// # 返回值
            /// 返回 `usize`，表示精度长度。
            ///
            /// # 示例
            /// ```
            /// let coefficient = ResourceTypeCoefficient::new(Ratio::new(1, 10)).unwrap();
            /// let precision = coefficient.calculate_precision();
            /// ```
            pub fn calculate_precision(&self) -> usize {
                return self.resource_type.denom().to_string().len();
            }
        }

        impl fmt::Display for ResourceTypeCoefficient {
            /// 格式化 `ResourceTypeCoefficient` 为字符串
            ///
            /// 将 `ResourceTypeCoefficient` 转换为 "分子/分母" 的格式字符串。
            ///
            /// # 示例
            /// ```
            /// let coefficient = ResourceTypeCoefficient::new(Ratio::new(1, 2)).unwrap();
            /// println!("{}", coefficient); // 输出 "1/2"
            /// ```
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}/{}",
                    self.resource_type.numer(),
                    self.resource_type.denom()
                )
            }
        }
    }
}
