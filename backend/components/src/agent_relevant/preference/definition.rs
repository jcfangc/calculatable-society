use crate::agent_relevant::preference::types::PreferenceValue;
use std::collections::HashMap;
use std::fmt;
use types::components_types::shared::resource::resource_type_coefficient::ResourceTypeCoefficient;

/// `Preference` 结构体，用于管理资源类型到偏好值的映射
#[derive(Debug)]
pub struct Preference {
    preferences: HashMap<ResourceTypeCoefficient, PreferenceValue>, // 使用 PreferenceValue 作为偏好值
}

impl Preference {
    /// 初始化 `Preference` 实例
    ///
    /// 可以选择传入初始的偏好映射表。如果没有提供初始数据，则使用一个空的 `HashMap`。
    ///
    /// # 参数
    /// - `preferences`: 一个可选的 `HashMap`，将 `ResourceTypeCoefficient` 映射到 `PreferenceValue`。
    ///
    /// # 返回值
    /// 返回一个新的 `Preference` 实例。
    ///
    /// # 示例
    /// ```
    /// let preferences = Preference::new(None);
    /// ```
    fn new(preferences: Option<HashMap<ResourceTypeCoefficient, PreferenceValue>>) -> Self {
        Preference {
            preferences: preferences.unwrap_or_else(HashMap::new),
        }
    }

    /// 添加或更新一个偏好项
    ///
    /// 为特定的资源类型设置或更新偏好值。如果该资源类型已经存在，则更新其偏好值；如果不存在，则添加新的资源类型及其偏好值。
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数，作为偏好映射表的键。
    /// - `preference_value`: 偏好值，表示对该资源类型的偏好程度。
    ///
    /// # 示例
    /// ```
    /// let mut preference = Preference::new(None);
    /// preference.set(ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap(), PreferenceValue::new(0.8).unwrap());
    /// ```
    fn set(&mut self, resource_type: ResourceTypeCoefficient, preference_value: PreferenceValue) {
        self.preferences.insert(resource_type, preference_value);
    }

    /// 获取特定资源类型的偏好值
    ///
    /// 根据资源类型系数，返回对应的偏好值。如果该资源类型不存在，则返回 `None`。
    ///
    /// # 参数
    /// - `resource_type`: 资源类型系数，作为查询的键。
    ///
    /// # 返回值
    /// 返回一个 `Option<&PreferenceValue>`，如果存在对应的资源类型则返回偏好值的引用，否则返回 `None`。
    ///
    /// # 示例
    /// ```
    /// let preference_value = preference.get(&ResourceTypeCoefficient::new(Ratio::new(1, 1)).unwrap());
    /// if let Some(value) = preference_value {
    ///     println!("偏好值: {}", value);
    /// }
    /// ```
    fn get(&self, resource_type: &ResourceTypeCoefficient) -> Option<&PreferenceValue> {
        self.preferences.get(resource_type)
    }
}

impl fmt::Display for Preference {
    /// 格式化 `Preference` 为字符串
    ///
    /// 将偏好映射表以字符串形式输出。
    ///
    /// # 示例
    /// ```
    /// let preference = Preference::new(None);
    /// println!("{}", preference); // 输出 "{}"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.preferences)
    }
}
