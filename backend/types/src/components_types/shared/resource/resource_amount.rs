use std::fmt;
use std::ops::{AddAssign, SubAssign};
use tracing::error;
use validator::ValidationError;

// 自定义 "资源数量" 类型并进行验证
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceAmount {
    resource_amount: i64, // 表示资源的数量
}

impl AddAssign for ResourceAmount {
    /// 重载 `+=` 运算符，实现资源数量的加法
    ///
    /// 将两个 `ResourceAmount` 实例相加，将结果赋值给左侧的实例。
    ///
    /// # 参数
    /// - `other`: 另一个 `ResourceAmount` 实例
    ///
    /// # 示例
    /// ```
    /// let mut amount = ResourceAmount::new(10).unwrap();
    /// amount += ResourceAmount::new(5).unwrap();
    ///
    /// assert_eq!(amount, ResourceAmount::new(15).unwrap());
    /// ```
    fn add_assign(&mut self, other: Self) {
        self.resource_amount += other.resource_amount;
    }
}

impl SubAssign for ResourceAmount {
    /// 重载 `-=` 运算符，实现资源数量的减法
    ///
    /// 将两个 `ResourceAmount` 实例相减，将结果赋值给左侧的实例。
    ///
    /// # 参数
    /// - `other`: 另一个 `ResourceAmount` 实例
    ///
    /// # 示例
    /// ```
    /// let mut amount = ResourceAmount::new(10).unwrap();
    /// amount -= ResourceAmount::new(5).unwrap();
    ///
    /// assert_eq!(amount, ResourceAmount::new(5).unwrap());
    /// ```
    fn sub_assign(&mut self, other: Self) {
        self.resource_amount -= other.resource_amount;
    }
}

impl ResourceAmount {
    /// 构造函数，创建 `ResourceAmount` 实例
    ///
    /// 资源数量在创建时必须经过验证，确保其为非负数。如果小于0，将返回验证错误。
    ///
    /// # 参数
    /// - `resource_amount`: 资源的数量，使用 `f64` 表示。
    ///
    /// # 返回值
    /// 返回一个 `Result<Self, ValidationError>`，其中 `Self` 是构造成功的 `ResourceAmount` 实例，`ValidationError` 则是当数量小于0时返回的错误。
    ///
    /// # 错误
    /// 当 `resource_amount` 小于 0.0 时，会返回 `ValidationError`。
    ///
    /// # 示例
    /// ```
    /// let amount = ResourceAmount::new(10).unwrap();
    /// ```
    fn new(resource_amount: i64) -> Result<Self, ValidationError> {
        if resource_amount < 0 {
            error!("资源数量不能为负数！");
            return Err(ValidationError::new("资源数量不能为负数！"));
        }
        Ok(ResourceAmount { resource_amount })
    }
}

impl fmt::Display for ResourceAmount {
    /// 格式化 `ResourceAmount` 为字符串
    ///
    /// 将 `ResourceAmount` 转换为表示数量的字符串。
    ///
    /// # 示例
    /// ```
    /// let amount = ResourceAmount::new(10).unwrap();
    /// println!("{}", amount); // 输出 "10"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.resource_amount)
    }
}
