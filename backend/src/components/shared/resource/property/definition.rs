use crate::components::shared::resource::types::ResourceTypeCoefficient;
use num::traits::ToPrimitive;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::f64::consts::PI;

/// `Property` 结构体，用于描述物质的属性
///
/// 每个属性通过频率常量和相位常量，以及环境因子来描述其物理/化学性质。
/// - `frequency_constant`: 属性频率的基础常量（a）
/// - `phase_constant`: 属性相位的基础常量（b）
/// - `environment_frequency_factor`: 环境频率因子（c），用于根据环境影响动态调整频率
/// - `environment_phase_factor`: 环境相位因子（d），用于根据环境影响动态调整相位
#[derive(Debug, Copy, Clone)]
pub struct PropertyConst {
    pub name: &'static str,
    pub frequency_constant: f64,           // 频率常量 a
    pub phase_constant: f64,               // 相位常量 b
    pub environment_frequency_factor: f64, // 环境频率因子 c
    pub environment_phase_factor: f64,     // 环境相位因子 d
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
    /// let property = PropertyConst::new(1.0, 0.0);
    /// ```
    pub const fn new(name: &'static str, frequency_constant: f64, phase_constant: f64) -> Self {
        PropertyConst {
            name,
            frequency_constant,
            phase_constant,
            environment_frequency_factor: 0.0,
            environment_phase_factor: 0.0,
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
    /// let property = PropertyConst::new(1.0, 0.0).with_frequency(2.0);
    /// ```
    pub fn with_env_frequency(mut self, env_frequency: f64) -> Self {
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
    /// let property = PropertyConst::new(1.0, 0.0).with_phase(0.5);
    /// ```
    pub fn with_env_phase(mut self, env_phase: f64) -> Self {
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
        let result = ((self.frequency_constant + self.environment_frequency_factor) * theta
            + (self.phase_constant + self.environment_phase_factor))
            .sin();

        return result;
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }
}

/// `property_const!` 宏，用于生成具有频率和相位常量的属性常量
///
/// # 参数
/// - `$name`: 属性名称（如 `FLAMMABLE`, `TOXIC`）
/// - `$a`: 频率常量（a）
/// - `$b`: 相位常量（b）
///
/// 生成的 `PropertyConst` 常量可以直接用于计算特定资源类型的属性值。
macro_rules! property_const {
    ($name:ident, $a:expr, $b:expr) => {
        pub const $name: PropertyConst =
            PropertyConst::new(stringify!($name), $a as f64, $b as f64);
    };
}

/// 辅助宏 `properties_impl` 递归定义属性，并递增频率和相位常量
///
/// # 参数
/// - `$a`: 当前频率常量（从 1 开始）
/// - `$b`: 当前相位常量（从 1 开始）
/// - `$name`: 当前处理的属性名称
/// - `$($rest)`: 其余需要处理的属性名称列表
///
/// 该宏用于 `properties!` 宏的内部递归调用，通过自动递增频率和相位常量来定义一系列属性常量。
macro_rules! properties_impl {
    // 递归处理多个属性名：每次递增频率和相位常量
    ($a:expr, $b:expr, $name:ident, $($rest:ident),+) => {
        property_const!($name, $a, $b);  // 定义当前属性
        properties_impl!($a + 1, $b + 1, $($rest),+);  // 递归调用，频率和相位常量递增
    };
    // 基本情况：只处理一个属性
    ($a:expr, $b:expr, $name:ident) => {
        property_const!($name, $a, $b);  // 定义最后一个属性
    };
}

/// `properties_const!` 宏，用于定义一系列属性常量
///
/// # 参数
/// - `$($name)`: 依次传入的属性名称列表（如 `FLAMMABLE`, `TOXIC`, `REACTIVE`）
///
/// 该宏从频率和相位常量都为 1 开始，递增定义所有传入的属性名称。
///
/// # 示例
/// ```
/// properties!(FLAMMABLE, TOXIC, REACTIVE, CORROSIVE);
/// ```
/// 定义了 4 个属性常量：`FLAMMABLE`, `TOXIC`, `REACTIVE`, `CORROSIVE`。
macro_rules! properties_const {
    // 递归调用处理多个属性名，从频率和相位常量为 1 开始
    ($($name:ident),+) => {
        properties_impl!(1, 1, $($name),+);
    };
}

/// 定义 `generate_property_map!` 宏，用于生成全局 `PROPERTIES` 并批量插入属性常量
macro_rules! generate_property_map {
    ($($name:ident),+) => {
        pub static PROPERTIES: Lazy<HashMap<&'static str, PropertyConst>> = Lazy::new(|| {
            let mut map = HashMap::new();
            $(map.insert(stringify!($name), $name);)+
            map
        });
    };
}

/// `define_and_register_properties!` 宏，整合 `properties_const!` 和 `generate_property_map!`
///
/// # 参数
/// - `$($name)`: 依次传入的属性名称列表（如 `FLAMMABLE`, `TOXIC`, `REACTIVE`）
///
/// 该宏先定义所有属性常量，然后将这些常量批量插入到全局 `PROPERTIES` 中
macro_rules! define_and_register_properties {
    ($($name:ident),+) => {
        // 调用 `properties_const!` 宏定义属性常量
        properties_const!($($name),+);

        // 调用 `generate_property_map!` 宏生成 `PROPERTIES`
        generate_property_map!($($name),+);
    };
}

// 使用 `define_and_register_properties!` 宏定义一系列属性常量
define_and_register_properties!(
    FLAMMABLE,   // 可燃性
    TOXIC,       // 毒性
    REACTIVE,    // 反应性
    CORROSIVE,   // 腐蚀性
    OXIDIZER,    // 氧化性
    ACID_BASE,   // 酸碱性
    PHASE,       // 相态
    CONDUCTIVE,  // 导电性
    MAGNETIC,    // 磁性
    BRITTLE,     // 脆性
    MALLEABLE,   // 延展性
    ELASTIC,     // 弹性
    TRANSPARENT  // 透明性
);
