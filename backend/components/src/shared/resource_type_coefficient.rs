use num::rational::Ratio;
use std::fmt;
use validator::ValidationError;

// 使用 const 来定义编译期常量上下界
const LOWER_BOUND: Ratio<usize> = Ratio::new_raw(0, 1); // 下限 0
const UPPER_BOUND: Ratio<usize> = Ratio::new_raw(2, 1); // 上限 2

// 自定义 "资源角系数" 类型，封装 Ratio 并进行验证
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ResourceTypeCoefficient {
    pub resource_type: Ratio<usize>, // 使用无符号整数，表示资源的种类及精密度
}

impl ResourceTypeCoefficient {
    /// 构造函数，使用分子和分母创建 `ResourceTypeCoefficient` 实例
    ///
    /// ### 参数
    /// - `numerator`: 分子，使用 `usize` 表示。
    /// - `denominator`: 分母，使用 `usize` 表示。
    ///
    /// ### 返回值
    /// 返回一个 `Result<Self, ValidationError>`，其中 `Self` 是构造成功的 `ResourceTypeCoefficient` 实例，`ValidationError` 则是当分数超出有效范围或无效时返回的错误。
    pub fn new(numerator: usize, denominator: usize) -> Result<Self, ValidationError> {
        if denominator == 0 {
            return Err(ValidationError::new("分母不能为零"));
        }

        let resource_type = Ratio::new(numerator, denominator); // 生成最简分数

        // 检查分数是否在指定范围内
        if resource_type < LOWER_BOUND || resource_type > UPPER_BOUND {
            tracing::error!(
                "{} 是非法的资源种类（资源角系数），有效范围为 {} 到 {}！",
                resource_type,
                LOWER_BOUND,
                UPPER_BOUND
            );
            return Err(ValidationError::new("非法的资源种类（资源角系数）！"));
        }

        Ok(ResourceTypeCoefficient { resource_type })
    }

    /// 计算资源种类的精密度
    pub fn calculate_precision(&self) -> usize {
        self.resource_type.denom().to_string().len()
    }
}

impl fmt::Display for ResourceTypeCoefficient {
    /// 格式化 `ResourceTypeCoefficient` 为字符串
    ///
    /// 将 `ResourceTypeCoefficient` 转换为 "分子/分母" 的格式字符串。
    ///
    /// ### 示例
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
