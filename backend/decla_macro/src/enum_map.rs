/// `enum_map` 宏用于定义一个枚举，并将该枚举与对应的值映射到一个静态的哈希表中。
///
/// 宏的使用格式为：
///
/// ```
/// enum_map! {
///     /// 文档注释
///     pub EnumName => ValueType {
///         Key1 => value_fn1,
///         Key2 => value_fn2,
///         Key3 => value_fn3,
///     }
/// }
/// ```
///
/// - `EnumName` 是枚举的名称。
/// - `ValueType` 是哈希表中枚举值对应的类型。
/// - `Key1`, `Key2`, `Key3` 等是枚举的变体，
///   宏生成的枚举会自动实现 `Debug`, `PartialEq`, `Eq`, 和 `Hash`。
/// - `value_fn1`, `value_fn2`, `value_fn3` 是与各个枚举变体对应的函数，它们的返回类型应当是 `ValueType`。
///
/// 该宏生成的 `to_map` 方法可以返回一个静态的 `HashMap`，
/// 其中包含枚举变体到 `ValueType` 的映射。
///
/// # 示例
///
/// ```rust
/// enum_map! {
///     pub ExampleEnum => i32 {
///         A => || 1,
///         B => || 2,
///         C => || 3,
///     }
/// }
///
/// fn main() {
///     let map = ExampleEnum::to_map();
///     assert_eq!(map.get(&ExampleEnum::A), Some(&1));
///     assert_eq!(map.get(&ExampleEnum::B), Some(&2));
///     assert_eq!(map.get(&ExampleEnum::C), Some(&3));
/// }
/// ```
#[macro_export]
macro_rules! enum_map {
    ($(#[$meta:meta])* $vis:vis $name:ident => $value_type:ty { $($key:ident => $value:expr),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, PartialEq, Eq, Hash)]
        $vis enum $name {
            $($key),*
        }

        use std::collections::HashMap;
        use once_cell::sync::Lazy;
        impl $name {
            $vis fn to_map() -> &'static HashMap<$name, $value_type> {
                static MAP: Lazy<HashMap<$name, $value_type>> = Lazy::new(
                    || {
                        let mut map = HashMap::new();
                        $(map.insert($name::$key, $value());)*
                        map
                    }
                );
                &MAP
            }
        }
    };
}
