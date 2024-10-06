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
