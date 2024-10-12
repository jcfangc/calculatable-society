use macros::PropertyMap;

/// 枚举 `Property`，表示物质的属性
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PropertyMap)]
pub enum Property {
    Flammable = 1, // 可燃性
    Toxic,         // 毒性
    Reactive,      // 反应性
    Corrosive,     // 腐蚀性
    Oxidizer,      // 氧化性
    AcidBase,      // 酸碱性
    Phase,         // 相态
    Conductive,    // 导电性
    Magnetic,      // 磁性
    Brittle,       // 脆性
    Malleable,     // 延展性
    Elastic,       // 弹性
    Transparent,   // 透明性
}
