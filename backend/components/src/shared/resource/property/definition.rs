use macros::PropertyMap;
use std::fmt::Display;

// /// `property_const!` 宏，用于生成具有频率和相位常量的属性常量
// ///
// /// # 参数
// /// - `$name`: 属性名称（如 `FLAMMABLE`, `TOXIC`）
// /// - `$a`: 频率常量（a）
// /// - `$b`: 相位常量（b）
// ///
// /// 生成的 `PropertyConst` 常量可以直接用于计算特定资源类型的属性值。
// macro_rules! property_const {
//     ($name:ident, $a:expr, $b:expr) => {
//         pub const $name: PropertyConst =
//             PropertyConst::new(stringify!($name), $a as f64, $b as f64);
//     };
// }

// /// 辅助宏 `properties_impl` 递归定义属性，并递增频率和相位常量
// ///
// /// # 参数
// /// - `$a`: 当前频率常量（从 1 开始）
// /// - `$b`: 当前相位常量（从 1 开始）
// /// - `$name`: 当前处理的属性名称
// /// - `$($rest)`: 其余需要处理的属性名称列表
// ///
// /// 该宏用于 `properties!` 宏的内部递归调用，通过自动递增频率和相位常量来定义一系列属性常量。
// macro_rules! properties_impl {
//     // 递归处理多个属性名：每次递增频率和相位常量
//     ($a:expr, $b:expr, $name:ident, $($rest:ident),+) => {
//         property_const!($name, $a, $b);  // 定义当前属性
//         properties_impl!($a + 1, $b + 1, $($rest),+);  // 递归调用，频率和相位常量递增
//     };
//     // 基本情况：只处理一个属性
//     ($a:expr, $b:expr, $name:ident) => {
//         property_const!($name, $a, $b);  // 定义最后一个属性
//     };
// }

// /// `properties_const!` 宏，用于定义一系列属性常量
// ///
// /// # 参数
// /// - `$($name)`: 依次传入的属性名称列表（如 `FLAMMABLE`, `TOXIC`, `REACTIVE`）
// ///
// /// 该宏从频率和相位常量都为 1 开始，递增定义所有传入的属性名称。
// ///
// /// # 示例
// /// ```
// /// properties!(FLAMMABLE, TOXIC, REACTIVE, CORROSIVE);
// /// ```
// /// 定义了 4 个属性常量：`FLAMMABLE`, `TOXIC`, `REACTIVE`, `CORROSIVE`。
// macro_rules! properties_const {
//     // 递归调用处理多个属性名，从频率和相位常量为 1 开始
//     ($($name:ident),+) => {
//         properties_impl!(1, 1, $($name),+);
//     };
// }

// /// 定义 `generate_property_map!` 宏，用于生成全局 `PROPERTIES` 并批量插入属性常量
// macro_rules! generate_property_map {
//     ($($name:ident),+) => {
//         pub static PROPERTIES: Lazy<HashMap<&'static str, PropertyConst>> = Lazy::new(|| {
//             let mut map = HashMap::new();
//             $(map.insert(stringify!($name), $name);)+
//             map
//         });
//     };
// }

// /// `define_and_register_properties!` 宏，整合 `properties_const!` 和 `generate_property_map!`
// ///
// /// # 参数
// /// - `$($name)`: 依次传入的属性名称列表（如 `FLAMMABLE`, `TOXIC`, `REACTIVE`）
// ///
// /// 该宏先定义所有属性常量，然后将这些常量批量插入到全局 `PROPERTIES` 中
// macro_rules! define_and_register_properties {
//     ($($name:ident),+) => {
//         // 调用 `properties_const!` 宏定义属性常量
//         properties_const!($($name),+);

//         // 调用 `generate_property_map!` 宏生成 `PROPERTIES`
//         generate_property_map!($($name),+);
//     };
// }

// // 使用 `define_and_register_properties!` 宏定义一系列属性常量
// define_and_register_properties!(
//     FLAMMABLE,   // 可燃性
//     TOXIC,       // 毒性
//     REACTIVE,    // 反应性
//     CORROSIVE,   // 腐蚀性
//     OXIDIZER,    // 氧化性
//     ACID_BASE,   // 酸碱性
//     PHASE,       // 相态
//     CONDUCTIVE,  // 导电性
//     MAGNETIC,    // 磁性
//     BRITTLE,     // 脆性
//     MALLEABLE,   // 延展性
//     ELASTIC,     // 弹性
//     TRANSPARENT  // 透明性
// );

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
