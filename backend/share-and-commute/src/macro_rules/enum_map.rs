// Macros 模块用于定义全局宏规则，方便在项目中复用代码片段，减少重复代码。
// 可以定义一些通用的宏，例如自动生成日志记录、统一错误处理等。
// 例如：macro_rules! log_error { (err:expr) => { println!("Error: {:?}", err); }; }

/// `enum_map` 宏用于定义一个枚举，并将该枚举与对应的值映射到一个静态的哈希表中。
///
/// 宏的使用格式为：
///
/// ```
/// enum_map! {
///     #[derive(Debug, PartialEq, Eq, Hash)] // 可以在这里添加需要的额外属性
///     pub(crate) EnumName => ValueType {
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
/// ### 示例
///
/// ```rust
/// enum_map! {
///     pub(crate) ExampleEnum => i32 {
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

            use once_cell::sync::Lazy;
            use std::collections::HashMap;

        // 将 MAP 定义在模块级别，移出 impl 块
        $vis static MAP: Lazy<HashMap<$name, $value_type>> = Lazy::new(|| {
            let mut map = HashMap::new();
            $(map.insert($name::$key, $value());)*
            map
        });

        impl $name {
            // 提供一个方法来访问静态的 HashMap
            $vis fn to_map() -> &'static HashMap<$name, $value_type> {
                &MAP
            }
        }
        };
    }
