use crate::agent::preference_value::PreferenceValue;
use crate::shared::subtance_type::SubtanceType;
use std::collections::HashMap;
use std::fmt;

/// `Preference` 结构体，用于管理资源类型到偏好值的映射
#[derive(Debug)]
pub struct Preferences {
    preferences: HashMap<SubtanceType, PreferenceValue>, // 使用 PreferenceValue 作为偏好值
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            preferences: HashMap::new(),
        }
    }
}

impl Preferences {
    /// 初始化 `Preference` 实例
    ///
    /// 可以选择传入初始的偏好映射表。如果没有提供初始数据，则使用一个空的 `HashMap`。
    ///
    /// ### 参数
    /// - `preferences`: 一个可选的 `HashMap`，将 `ResourceTypeCoefficient` 映射到 `PreferenceValue`。
    ///
    /// ### 返回值
    /// 返回一个新的 `Preference` 实例。
    ///
    /// ### 示例
    /// ```
    /// let preferences = Preference::new(None);
    /// ```
    fn new(preferences: Option<HashMap<SubtanceType, PreferenceValue>>) -> Self {
        Preferences {
            preferences: preferences.unwrap_or_default(),
        }
    }

    /// 添加或更新一个偏好项
    ///
    /// 为特定的资源类型设置或更新偏好值。如果该资源类型已经存在，则更新其偏好值；如果不存在，则添加新的资源类型及其偏好值。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数，作为偏好映射表的键。
    /// - `preference_value`: 偏好值，表示对该资源类型的偏好程度。
    ///
    /// ### 示例
    /// ```
    /// let mut preference = Preference::new(None);
    /// preference.set(ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap(), PreferenceValue::new(0.8).unwrap());
    /// ```
    fn set(&mut self, resource_type: SubtanceType, preference_value: PreferenceValue) {
        self.preferences.insert(resource_type, preference_value);
    }

    /// 获取特定资源类型的偏好值
    ///
    /// 根据资源类型系数，返回对应的偏好值。如果该资源类型不存在，则返回 `None`。
    ///
    /// ### 参数
    /// - `resource_type`: 资源类型系数，作为查询的键。
    ///
    /// ### 返回值
    /// 返回一个 `Option<&PreferenceValue>`，如果存在对应的资源类型则返回偏好值的引用，否则返回 `None`。
    ///
    /// ### 示例
    /// ```
    /// let preference_value = preference.get(&ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap());
    /// if let Some(value) = preference_value {
    ///     println!("偏好值: {}", value);
    /// }
    /// ```
    fn get(&self, resource_type: &SubtanceType) -> Option<&PreferenceValue> {
        self.preferences.get(resource_type)
    }
}

impl fmt::Display for Preferences {
    /// 格式化 `Preference` 为字符串
    ///
    /// 将偏好映射表以字符串形式输出。
    ///
    /// ### 示例
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
    use super::model::PreferencesModel;
    use crate::agent::preference_value::PreferenceValue;
    use crate::shared::subtance_type::SubtanceType;
    use context::db::context::DatabaseContext;
    use context::GLOBAL_APP_CONTEXT;
    use sqlx::Error;
    use uuid::Uuid;

    /// 根据 `resource_numerator` 和 `resource_dominator` 从数据库获取 `PropertyModel`
    pub async fn get_properties_by_numerator_and_dominator(
        agent_id: Uuid,
        resource_numerator: i32,
        resource_dominator: i32,
    ) -> Result<(SubtanceType, PreferenceValue), Error> {
        let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

        // 使用 sqlx::query_as 函数版本
        if let Some(preferences_model) = sqlx::query_as::<_, PreferencesModel>(
            r#"
            SELECT 
                agent_id,
                numerator,
                denominator,
                preference
            FROM
                preferences
            WHERE 
                agent_id = $1 AND numerator = $2 AND denominator = $32
            "#,
        )
        .bind(agent_id)
        .bind(resource_numerator)
        .bind(resource_dominator)
        .fetch_optional(pool)
        .await?
        {
            if let Ok(resource_type_coefficient) = SubtanceType::try_new(
                preferences_model.numerator as usize,
                preferences_model.denominator as usize,
            ) {
                if let Ok(preference_value) = PreferenceValue::try_new(preferences_model.preference)
                {
                    return Ok((resource_type_coefficient, preference_value));
                }
            }
        }

        Err(Error::RowNotFound)
    }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    use sqlx::FromRow;
    use uuid::Uuid;

    #[derive(FromRow)]
    pub struct PreferencesModel {
        pub agent_id: Uuid,
        pub numerator: i32,
        pub denominator: i32,
        pub preference: f64,
    }
}
