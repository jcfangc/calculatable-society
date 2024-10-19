use types::PropertyConst;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub Property => PropertyConst {
        Flammable => || PropertyConst::new(1,1),
        Toxic => || PropertyConst::new(2,2),
        Reactive => || PropertyConst::new(3,3),
        Corrosive => || PropertyConst::new(4,4),
        Oxidizer => || PropertyConst::new(5,5),
        AcidBase => || PropertyConst::new(6,6),
        Phase => || PropertyConst::new(7,7),
        Conductive => || PropertyConst::new(8,8),
        Magnetic => || PropertyConst::new(9,9),
        Brittle => || PropertyConst::new(10,10),
        Malleable => || PropertyConst::new(11,11),
        Elastic => || PropertyConst::new(12,12),
        Transparent => || PropertyConst::new(13,13),
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
mod repository {
    // Repository 模块专注于与数据库的交互，处理数据的持久化操作。
    // 例如: 通过数据库查询获取 property 数据，或者将新的 property 插入数据库。
    // 在这里定义具体的数据访问方法。
    // 例如: fn get_property_by_id(id: u32) -> Option<property> { ... }
}

// Model 模块: 负责定义数据模型，通常是数据库表的抽象结构
mod model {
    // 在这里定义你的数据结构。
    // 例如: struct property { ... }
}

// Types 模块: 封装与组件相关的基础类型，便于全局使用
mod types {
    use crate::shared::resource::types::resource_type_coefficient::ResourceTypeCoefficient;
    use num::traits::ToPrimitive;
    use std::f64::consts::PI;

    /// `PropertyConst` 结构体，用于描述物质的属性
    ///
    /// 每个属性通过频率常量和相位常量，以及环境因子来描述其物理/化学性质。
    /// - `frequency_constant`: 属性频率的基础常量（a）
    /// - `phase_constant`: 属性相位的基础常量（b）
    /// - `environment_frequency_factor`: 环境频率因子（c），用于根据环境影响动态调整频率
    /// - `environment_phase_factor`: 环境相位因子（d），用于根据环境影响动态调整相位
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct PropertyConst {
        pub frequency_constant: i32,           // 频率常量 a
        pub phase_constant: i32,               // 相位常量 b
        pub environment_frequency_factor: i32, // 环境频率因子 c
        pub environment_phase_factor: i32,     // 环境相位因子 d
    }

    impl PropertyConst {
        /// 构造函数，创建一个新的 `PropertyConst` 实例
        ///
        /// # 参数
        /// - `a`: 频率常量（a）
        /// - `b`: 相位常量（b）
        ///
        /// # 返回值
        /// 返回一个新的 `PropertyConst` 实例
        ///
        /// # 示例
        /// ```
        /// let property = PropertyConst::new(1, 0);
        /// ```
        pub fn new(frequency_constant: i32, phase_constant: i32) -> Self {
            PropertyConst {
                frequency_constant,
                phase_constant,
                environment_frequency_factor: 0,
                environment_phase_factor: 0,
            }
        }

        /// 设置频率常量（可链式调用）
        ///
        /// # 参数
        /// - `env_frequency`: 频率常量 `c`
        ///
        /// # 返回值
        /// 返回带有新的频率常量的 `PropertyConst` 实例，用于动态调整属性。
        ///
        /// # 示例
        /// ```
        /// let property = PropertyConst::new(1, 0).with_env_frequency(2);
        /// ```
        pub fn with_env_frequency(mut self, env_frequency: i32) -> Self {
            self.environment_frequency_factor = env_frequency;
            return self;
        }

        /// 设置相位常量（可链式调用）
        ///
        /// # 参数
        /// - `env_phase`: 相位常量 `d`
        ///
        /// # 返回值
        /// 返回带有新的相位常量的 `PropertyConst` 实例，用于动态调整属性。
        ///
        /// # 示例
        /// ```
        /// let property = PropertyConst::new(1, 0).with_env_phase(0);
        /// ```
        pub fn with_env_phase(mut self, env_phase: i32) -> Self {
            self.environment_phase_factor = env_phase;
            return self;
        }

        /// 计算属性值
        ///
        /// 根据资源类型系数来计算属性值，θ = 资源类型系数 × π
        ///
        /// # 参数
        /// - `coefficient`: 资源类型系数 `ResourceTypeCoefficient`，表示资源类型与属性的关系
        ///
        /// # 返回值
        /// 返回计算后的属性值，基于频率和相位常量与资源类型系数的组合。
        ///
        /// # 计算公式
        /// `sin((a + c)θ + (b + d))`
        ///
        /// 其中：
        /// - `θ = ResourceTypeCoefficient * π`
        /// - `a` 和 `b` 为基础频率和相位常量
        /// - `c` 和 `d` 为环境频率因子和相位因子
        pub fn calculate(&self, coefficient: &ResourceTypeCoefficient) -> f64 {
            // 计算 θ = 资源类型系数 × π
            let theta = coefficient.resource_type.to_f64().unwrap() * PI;

            // 计算 sin((a + c)θ + (b + d))
            let result = ((self.frequency_constant + self.environment_frequency_factor) as f64
                * theta
                + (self.phase_constant + self.environment_phase_factor) as f64)
                .sin();

            return result;
        }
    }
}
