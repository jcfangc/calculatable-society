use crate::shared::property::Property;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::fmt;
use types::resource_amount::{
    AllocatableOperation, DebtOperation, InvestmentOperation, ResourceAmount,
};
use types::resource_type_coefficient::ResourceTypeCoefficient;

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

    /// 获取资源的种类
    pub fn get_resource_type(&self) -> &ResourceTypeCoefficient {
        &self.resource_type
    }

    /// 获取资源的数量
    pub fn get_amount(&self) -> &ResourceAmount {
        &self.amount
    }

    /// 设置资源的数量
    pub fn set_amount(&mut self, new_allocatable: u64, new_investment: u64, new_debt: u64) {
        self.amount
            .set_allocatable(new_allocatable)
            .set_investment(new_investment)
            .set_debt(new_debt)
            .finalize();
    }

    /// 计算资源属性
    pub async fn properties(&self) -> HashMap<Property, f64> {
        // 直接使用 into_par_iter 进行并行计算，避免 Vec 转换
        let property_value_entries: HashMap<_, _> = Property::to_map()
            .await
            .into_par_iter() // 将 HashMap 转换为并行迭代器
            .map(|(property, property_const)| {
                (*property, property_const.calculate(&self.resource_type))
            })
            .collect(); // 将并行结果收集为 HashMap

        // 返回属性
        property_value_entries
    }
}

impl fmt::Display for Resource {
    /// 格式化 `Resource` 为字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.resource_type, self.amount)
    }
}

// resource 组件

// Route 模块: 定义路由，将 URL 映射到具体的处理方法
mod route {
    //Route 模块负责定义连接的路由规则，将 URL 映射到 Controller 中的处理方法。
    // 例如: Router::new().route("//resource", (handle_resource_))
}

// Controller 模块: 处理来自客户端的请求，负责连接的生命周期管理
mod controller {
    // Controller 负责处理连接的建立、消息收发以及关闭等生命周期操作。
    // 它接收来自客户端的消息，并将消息传递给 Service 进行业务处理。
    // 例如: async fn handle_resource_(socket: WebSocket) -> impl IntoResponse { ... }
}

// Service 模块: 负责处理消息的业务逻辑，调用 Repository 获取或更新数据
mod service {
    // Service 处理来自 Controller 的消息，执行具体的业务逻辑。
    // 例如: 处理收到的消息，更新数据库中的状态或发送响应消息。
    // 例如: async fn process_resource_message(message: String) -> Result<String, Error> { ... }
}

// Repository 模块: 负责与数据库的交互，执行数据的增删改查操作
mod repository {
    use super::model::ResourceModel;
    use context::db::context::DatabaseContext;
    use context::GLOBAL_APP_CONTEXT;

    pub async fn get_resource(
        numerator: u32,
        denominator: u32,
    ) -> Result<ResourceModel, sqlx::Error> {
        // 获取数据库连接池
        let pool = GLOBAL_APP_CONTEXT
            .get()
            .expect("AppContext 未初始化")
            .db_pool()
            .await;

        // 使用连接池执行查询
        let result = sqlx::query_as!(
            ResourceModel,
            "SELECT numerator as numerator: i32, denominator as denominator: i32, allocatable as allocatable: i64, investment as investment: i64, debt as debt: i64 FROM resource WHERE numerator = $1 AND denominator = $2",
            numerator as i32,
            denominator as i32
        )
        .fetch_optional(&*pool)
        .await?;

        if let Some(resource) = result {
            // 处理找到的数据
        } else {
            // 处理没有找到数据的情况
        }
    }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct ResourceModel {
        pub numerator: u32,
        pub denominator: u32,
        pub allocatable: u64,
        pub investment: u64,
        pub debt: u64,
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
            allocatable: u64, // 表示当前可分配的资源量
            investment: u64,  // 累积投资总量
            debt: u64,        // 累积债务总量
        }

        impl ResourceAmount {
            /// 构造函数，创建 `ResourceAmount` 实例
            pub fn new(allocatable: u64, investment: u64, debt: u64) -> Self {
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
            fn get_allocatable(&self) -> u64;
            fn set_allocatable(&mut self, new_value: u64) -> &mut Self;
        }

        impl AllocatableOperation for ResourceAmount {
            fn get_allocatable(&self) -> u64 {
                self.allocatable
            }

            fn set_allocatable(&mut self, new_value: u64) -> &mut Self {
                self.allocatable = new_value;
                self
            }
        }

        // 定义 investment 字段的相关行为
        pub trait InvestmentOperation {
            fn get_investment(&self) -> u64;
            fn set_investment(&mut self, new_value: u64) -> &mut Self;
        }

        impl InvestmentOperation for ResourceAmount {
            fn get_investment(&self) -> u64 {
                self.investment
            }

            fn set_investment(&mut self, new_value: u64) -> &mut Self {
                self.investment = new_value;
                self
            }
        }

        // 定义 debt 字段的相关行为
        pub trait DebtOperation {
            fn get_debt(&self) -> u64;
            fn set_debt(&mut self, new_value: u64) -> &mut Self;
        }

        impl DebtOperation for ResourceAmount {
            fn get_debt(&self) -> u64 {
                self.debt
            }

            fn set_debt(&mut self, new_value: u64) -> &mut Self {
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
