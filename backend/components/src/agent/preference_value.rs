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
    pub fn new(value: f64) -> Result<Self, String> {
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
