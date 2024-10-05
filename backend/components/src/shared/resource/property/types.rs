use crate::shared::resource::types::ResourceTypeCoefficient;
use num::traits::ToPrimitive;
use std::f64::consts::PI;

/// `Property` 结构体，用于描述物质的属性
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
    pub const fn new(frequency_constant: i32, phase_constant: i32) -> Self {
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
        let result = ((self.frequency_constant + self.environment_frequency_factor) as f64 * theta
            + (self.phase_constant + self.environment_phase_factor) as f64)
            .sin();

        return result;
    }
}
