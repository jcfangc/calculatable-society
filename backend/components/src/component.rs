use crate::agent::preferences::Preferences;
use crate::shared::resources::Resources;
use std::fmt;

/// `Component` 枚举，表示系统中可能存在的多种组件。
/// 目前有两种组件：`Resource` 和 `Preference`，但未来可能会随着开发扩展更多类型的组件。
pub enum Component {
    /// 资源组件，存储 `Resource` 结构体
    Resources(Resources),

    /// 偏好组件，存储 `Preference` 结构体
    Preferences(Preferences),
    // 未来可能添加更多组件类型
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Component::Resources(resource) => write!(f, "Resource({:#?})", resource),
            Component::Preferences(preference) => write!(f, "Preference({:#?})", preference),
        }
    }
}
