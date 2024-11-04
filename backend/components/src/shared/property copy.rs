use types::PropertyParam;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub Property => PropertyParam {
        Flammable => || PropertyParam::new(1,1),        // 可燃性
        Toxic => || PropertyParam::new(2,2),            // 有毒性
        Reactive => || PropertyParam::new(3,3),         // 反应性
        Corrosive => || PropertyParam::new(4,4),        // 腐蚀性
        Oxidizer => || PropertyParam::new(5,5),         // 氧化性
        AcidBase => || PropertyParam::new(6,6),         // 酸碱性
        Phase => || PropertyParam::new(7,7),            // 物态
        Conductive => || PropertyParam::new(8,8),       // 导电性
        Magnetic => || PropertyParam::new(9,9),         // 磁性
        Brittle => || PropertyParam::new(10,10),        // 易碎性
        Malleable => || PropertyParam::new(11,11),      // 可塑性
        Elastic => || PropertyParam::new(12,12),        // 弹性
        Transparent => || PropertyParam::new(13,13),    // 透明性
    }
}

// property 组件

// Route 模块: 定义路由，将 URL 映射到具体的处理方法
mod route {
    //Route 模块负责定义连接的路由规则，将 URL 映射到 Controller 中的处理方法。
    // 例如: Router::new().route("//property", (handle_property_))
}

// Controller 模块: 处理来自客户端的请求，负责连接的生命周期管理
mod controller {
    // Controller 负责处理连接的建立、消息收发以及关闭等生命周期操作。
    // 它接收来自客户端的消息，并将消息传递给 Service 进行业务处理。
    // 例如: async fn handle_property_(socket: WebSocket) -> impl IntoResponse { ... }
}

// Service 模块: 负责处理消息的业务逻辑，调用 Repository 获取或更新数据
mod service {
    // Service 处理来自 Controller 的消息，执行具体的业务逻辑。
    // 例如: 处理收到的消息，更新数据库中的状态或发送响应消息。
    // 例如: async fn process_property_message(message: String) -> Result<String, Error> { ... }
}

// Repository 模块: 负责与数据库的交互，执行数据的增删改查操作
pub mod repository {
    use super::model::PropertyModel;
    use super::Property;
    use context::db::context::DatabaseContext;
    use context::GLOBAL_APP_CONTEXT;
    use sqlx::Error;
    use std::collections::HashMap;

    /// 根据 `resource_numerator` 和 `resource_dominator` 从数据库获取 `PropertyModel`
    pub async fn get_properties_by_numerator_and_dominator(
        resource_numerator: i32,
        resource_dominator: i32,
    ) -> Result<HashMap<Property, f64>, Error> {
        let pool = &*GLOBAL_APP_CONTEXT.get().unwrap().db_pool().await;

        // 使用 sqlx::query_as 函数版本
        if let Some(property_model) = sqlx::query_as::<_, PropertyModel>(
            r#"
            SELECT 
                resource_numerator,
                resource_dominator,
                flammable, toxic, reactive, 
                corrosive, oxidizer, acid_base, 
                phase, conductive, magnetic, 
                brittle, malleable, elastic, 
                transparent
            FROM 
                properties
            WHERE 
                resource_numerator = $1 AND resource_dominator = $2
            "#,
        )
        .bind(resource_numerator)
        .bind(resource_dominator)
        .fetch_optional(pool)
        .await?
        {
            return Ok(property_model.to_map());
        }

        Err(Error::RowNotFound)
    }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    use super::Property;
    use sqlx::FromRow;
    use std::collections::HashMap;

    #[derive(FromRow)]
    pub struct PropertyModel {
        pub resource_numerator: i32,
        pub resource_dominator: i32,
        pub flammable: f64,
        pub toxic: f64,
        pub reactive: f64,
        pub corrosive: f64,
        pub oxidizer: f64,
        pub acid_base: f64,
        pub phase: f64,
        pub conductive: f64,
        pub magnetic: f64,
        pub brittle: f64,
        pub malleable: f64,
        pub elastic: f64,
        pub transparent: f64,
    }

    impl PropertyModel {
        /// 将 `PropertyModel` 转换为 `HashMap<Property, f64>`
        pub fn to_map(&self) -> HashMap<Property, f64> {
            let mut map = HashMap::new();
            map.insert(Property::Flammable, self.flammable);
            map.insert(Property::Toxic, self.toxic);
            map.insert(Property::Reactive, self.reactive);
            map.insert(Property::Corrosive, self.corrosive);
            map.insert(Property::Oxidizer, self.oxidizer);
            map.insert(Property::AcidBase, self.acid_base);
            map.insert(Property::Phase, self.phase);
            map.insert(Property::Conductive, self.conductive);
            map.insert(Property::Magnetic, self.magnetic);
            map.insert(Property::Brittle, self.brittle);
            map.insert(Property::Malleable, self.malleable);
            map.insert(Property::Elastic, self.elastic);
            map.insert(Property::Transparent, self.transparent);
            map
        }
    }
}

// Types 模块: 封装与组件相关的基础类型，便于全局使用
mod types {
    use crate::shared::subtance_type::SubtanceType;
    use num::traits::ToPrimitive;
    use std::f64::consts::PI;

    /// `PropertyParam` 结构体，用于描述物质的属性
    ///
    /// 每个属性通过频率常量和相位常量，以及环境因子来描述其物理/化学性质。
    /// - `frequency_constant`: 属性频率的基础常量（a）
    /// - `phase_constant`: 属性相位的基础常量（b）
    /// - `environment_frequency_factor`: 环境频率因子（c），用于根据环境影响动态调整频率
    /// - `environment_phase_factor`: 环境相位因子（d），用于根据环境影响动态调整相位
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PropertyParam {
        pub frequency_constant: isize, // 频率常量 a
        pub phase_constant: isize,     // 相位常量 b
        pub frequency_offset: isize,   // 环境频率因子 c
        pub phase_offset: isize,       // 环境相位因子 d
    }

    impl PropertyParam {
        /// 构造函数，创建一个新的 `PropertyParam` 实例
        ///
        /// ### 参数
        /// - `a`: 频率常量（a）
        /// - `b`: 相位常量（b）
        ///
        /// ### 返回值
        /// 返回一个新的 `PropertyParam` 实例
        ///
        /// ### 示例
        /// ```
        /// let property = PropertyParam::new(1, 0);
        /// ```
        pub fn new(frequency_constant: isize, phase_constant: isize) -> Self {
            PropertyParam {
                frequency_constant,
                phase_constant,
                frequency_offset: 0,
                phase_offset: 0,
            }
        }

        /// 设置频率常量（可链式调用）
        ///
        /// ### 参数
        /// - `env_frequency`: 频率常量 `c`
        ///
        /// ### 返回值
        /// 返回带有新的频率常量的 `PropertyParam` 实例，用于动态调整属性。
        ///
        /// ### 示例
        /// ```
        /// let property = PropertyParam::new(1, 0).with_env_frequency(2);
        /// ```
        pub fn with_frequency_offset(mut self, env_frequency: isize) -> Self {
            self.frequency_offset = env_frequency;
            return self;
        }

        /// 设置相位常量（可链式调用）
        ///
        /// ### 参数
        /// - `env_phase`: 相位常量 `d`
        ///
        /// ### 返回值
        /// 返回带有新的相位常量的 `PropertyParam` 实例，用于动态调整属性。
        ///
        /// ### 示例
        /// ```
        /// let property = PropertyParam::new(1, 0).with_env_phase(0);
        /// ```
        pub fn with_phase_offset(mut self, env_phase: isize) -> Self {
            self.phase_offset = env_phase;
            return self;
        }

        /// 计算属性值
        ///
        /// 根据资源类型系数来计算属性值，θ = 资源类型系数 × π
        ///
        /// ### 参数
        /// - `coefficient`: 资源类型系数 `ResourceTypeCoefficient`，表示资源类型与属性的关系
        ///
        /// ### 返回值
        /// 返回计算后的属性值，基于频率和相位常量与资源类型系数的组合。
        ///
        /// ### 计算公式
        /// `sin((a + c)θ + (b + d))`
        ///
        /// 其中：
        /// - `θ = ResourceTypeCoefficient * π`
        /// - `a` 和 `b` 为基础频率和相位常量
        /// - `c` 和 `d` 为环境频率因子和相位因子
        pub fn calculate(&self, coefficient: &SubtanceType) -> f64 {
            // 计算 θ = 资源类型系数 × π
            let theta = coefficient.subtance_type.to_f64().unwrap() * PI;

            // 计算 sin((a + c)θ + (b + d))
            let result = ((self.frequency_constant + self.frequency_offset) as f64 * theta
                + (self.phase_constant + self.phase_offset) as f64)
                .sin();

            return result;
        }
    }
}
