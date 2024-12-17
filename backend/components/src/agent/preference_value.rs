use error_handling::PreferenceValueError;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt;

/// `PreferenceValue` 结构体，用于表示一个在 0 到 1 之间的偏好值
#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct PreferenceValue {
    value: f64,
}

impl PreferenceValue {
    /// 智能构造函数 `try_new`，创建一个 `PreferenceValue` 实例
    ///
    /// 在构造时会验证输入值是否在 0 到 1 之间。
    ///
    /// ### 参数
    /// - `value`: 偏好值，应该在 0.0 到 1.0 之间。
    ///
    /// ### 返回值
    /// 返回一个 `Result<Self, PreferenceValueError>`，其中 `Self` 是成功构造的 `PreferenceValue` 实例，
    /// `PreferenceValueError` 是验证失败时的错误类型。
    pub(crate) fn try_new(value: f64) -> Result<Self, PreferenceValueError> {
        if value >= 0.0 && value <= 1.0 {
            Ok(PreferenceValue { value })
        } else {
            tracing::error!("偏好值 {} 不在 0 到 1 之间！", value);
            Err(PreferenceValueError::OutOfRange)
        }
    }

    /// 获取偏好值
    pub(crate) fn get_value(&self) -> f64 {
        self.value
    }
}

/// 实现 `TryFrom<f64>`，便于使用 `try_into` 方法构造 `PreferenceValue`
impl TryFrom<f64> for PreferenceValue {
    type Error = PreferenceValueError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        PreferenceValue::try_new(value)
    }
}

impl fmt::Display for PreferenceValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(_) => write!(f, "PreferenceValue {{ value: {} }}", self.value),
        }
    }
}

pub(crate) mod error_handling {
    use std::fmt;

    /// `PreferenceValueError` 枚举，用于表示构造 `PreferenceValue` 时的错误类型
    #[derive(Debug)]
    pub(crate) enum PreferenceValueError {
        OutOfRange,
    }

    impl fmt::Display for PreferenceValueError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PreferenceValueError::OutOfRange => write!(f, "偏好值必须在 0 到 1 之间"),
            }
        }
    }
}
