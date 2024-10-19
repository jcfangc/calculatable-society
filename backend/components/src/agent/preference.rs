use crate::shared::resource::types::resource_type_coefficient::ResourceTypeCoefficient;
use std::collections::HashMap;
use std::fmt;
use types::PreferenceValue;

/// `Preference` 结构体，用于管理资源类型到偏好值的映射
#[derive(Debug)]
pub struct Preference {
    preferences: HashMap<ResourceTypeCoefficient, PreferenceValue>, // 使用 PreferenceValue 作为偏好值
}

impl Preference {
    /// 初始化 `Preference` 实例
    ///
    /// 可以选择传入初始的偏好映射表。如果没有提供初始数据，则使用一个空的 `HashMap`。
    ///
    /// # 参数
    /// - `preferences`: 一个可选的 `HashMap`，将 `ResourceTypeCoefficient` 映射到 `PreferenceValue`。
    ///
    /// # 返回值
    /// 返回一个新的 `Preference` 实例。
    ///
    /// # 示例
    /// ```
    /// let preferences = Preference::new(None);
    /// ```
    fn new(preferences: Option<HashMap<ResourceTypeCoefficient, PreferenceValue>>) -> Self {
        Preference {
            preferences: preferences.unwrap_or_else(HashMap::new),
        }
    }

    /// 添加或更新一个偏好项
    ///
    /// 为特定的资源类型设置或更新偏好值。如果该资源类型已经存在，则更新其偏好值；如果不存在，则添加新的资源类型及其偏好值。
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数，作为偏好映射表的键。
    /// - `preference_value`: 偏好值，表示对该资源类型的偏好程度。
    ///
    /// # 示例
    /// ```
    /// let mut preference = Preference::new(None);
    /// preference.set(ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap(), PreferenceValue::new(0.8).unwrap());
    /// ```
    fn set(&mut self, resource_type: ResourceTypeCoefficient, preference_value: PreferenceValue) {
        self.preferences.insert(resource_type, preference_value);
    }

    /// 获取特定资源类型的偏好值
    ///
    /// 根据资源类型系数，返回对应的偏好值。如果该资源类型不存在，则返回 `None`。
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数，作为查询的键。
    ///
    /// # 返回值
    /// 返回一个 `Option<&PreferenceValue>`，如果存在对应的资源类型则返回偏好值的引用，否则返回 `None`。
    ///
    /// # 示例
    /// ```
    /// let preference_value = preference.get(&ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap());
    /// if let Some(value) = preference_value {
    ///     println!("偏好值: {}", value);
    /// }
    /// ```
    fn get(&self, resource_type: &ResourceTypeCoefficient) -> Option<&PreferenceValue> {
        self.preferences.get(resource_type)
    }
}

impl fmt::Display for Preference {
    /// 格式化 `Preference` 为字符串
    ///
    /// 将偏好映射表以字符串形式输出。
    ///
    /// # 示例
    /// ```
    /// let preference = Preference::new(None);
    /// println!("{}", preference); // 输出 "{}"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.preferences)
    }
}

// preference 组件

// Route 模块: 定义路由，将 URL 映射到具体的处理方法
mod route {
    //Route 模块负责定义连接的路由规则，将 URL 映射到 Controller 中的处理方法。
    // 例如: Router::new().route("//preference", (handle_preference_))
}

// Controller 模块: 处理来自客户端的请求，负责连接的生命周期管理
mod controller {
    // Controller 负责处理连接的建立、消息收发以及关闭等生命周期操作。
    // 它接收来自客户端的消息，并将消息传递给 Service 进行业务处理。
    // 例如: async fn handle_preference_(socket: WebSocket) -> impl IntoResponse { ... }
}

// Service 模块: 负责处理消息的业务逻辑，调用 Repository 获取或更新数据
mod service {
    // Service 处理来自 Controller 的消息，执行具体的业务逻辑。
    // 例如: 处理收到的消息，更新数据库中的状态或发送响应消息。
    // 例如: async fn process_preference_message(message: String) -> Result<String, Error> { ... }
}

// Repository 模块: 负责与数据库的交互，执行数据的增删改查操作
mod repository {
    // Repository 模块专注于与数据库的交互，处理数据的持久化操作。
    // 例如: 通过数据库查询获取 preference 数据，或者将新的 preference 插入数据库。
    // 在这里定义具体的数据访问方法。
    // 例如: fn get_preference_by_id(id: u32) -> Option<preference> { ... }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    // 在这里定义你的数据结构。
    // 例如: struct preference { ... }
}

// Types 模块: 封装与组件相关的基础类型，便于全局使用
mod types {
    use std::fmt;
    use tracing::error;

    /// `PreferenceValue` 结构体，用于表示一个在 0 到 1 之间的偏好值
    #[derive(Debug, Clone, Copy)]
    pub struct PreferenceValue {
        value: f64,
    }

    impl PreferenceValue {
        /// 构造函数，创建一个 `PreferenceValue` 实例
        ///
        /// 在构造时会验证输入值是否在 0 到 1 之间。如果值不符合要求，将返回错误信息。
        ///
        /// # 参数
        /// - `value`: 偏好值，应该在 0.0 到 1.0 之间。
        ///
        /// # 返回值
        /// 返回一个 `Result<Self, String>`，其中 `Self` 是成功构造的 `PreferenceValue` 实例，`String` 是验证失败时的错误信息。
        ///
        /// # 错误
        /// 当 `value` 不在 0.0 到 1.0 之间时，会返回一个错误信息。
        ///
        /// # 示例
        /// ```
        /// let preference_value = PreferenceValue::new(0.8).unwrap();
        /// let invalid_value = PreferenceValue::new(1.5); // 结果会是一个错误
        /// ```
        fn new(value: f64) -> Result<Self, String> {
            if value >= 0.0 && value <= 1.0 {
                Ok(PreferenceValue { value })
            } else {
                error!("偏好值 {} 不在 0 到 1 之间！", value);
                Err(format!("偏好值 {} 不在 0 到 1 之间！", value))
            }
        }

        /// 获取偏好值
        ///
        /// 返回当前实例中的偏好值。
        ///
        /// # 返回值
        /// 返回一个 `f64`，表示偏好值。
        ///
        /// # 示例
        /// ```
        /// let preference_value = PreferenceValue::new(0.5).unwrap();
        /// let value = preference_value.get_value();
        /// ```
        fn get_value(&self) -> f64 {
            self.value
        }
    }

    impl fmt::Display for PreferenceValue {
        /// 格式化 `PreferenceValue` 为字符串
        ///
        /// 将偏好值以字符串形式输出。
        ///
        /// # 示例
        /// ```
        /// let preference_value = PreferenceValue::new(0.75).unwrap();
        /// println!("{}", preference_value); // 输出 "0.75"
        /// ```
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }
}
