// utils 模块：用于定义中间件、基础类型封装和全局使用的宏规则

// Middleware 模块: 定义全局的中间件，用于处理连接和消息的前置和后置操作
mod middleware {
    // Middleware 模块负责处理请求或消息的前置、后置操作，例如日志记录、错误处理或身份验证。
    // 可以定义通用的中间件函数，在连接或消息处理时调用。
    // 例如：async fn log_middleware(socket: WebSocket, next: impl FnOnce() -> ()) -> Result<(), Error> { ... }
}

// Types 模块: 封装基础类型，便于全局使用
mod types {
    // Types 模块封装基础类型，例如 ID、UUID、时间戳等，统一管理类型。
    // 例如: pub(crate) struct UserId(pub(crate) u32);
    // 在项目中通过定义新类型来区分不同用途的基础类型，增强代码的可读性和安全性。
    // 例如: pub(crate) struct Timestamp(pub(crate) i64);
}

// Macros 模块: 定义全局宏规则，简化代码编写
pub(crate) mod macros {

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
}
// Utils 模块的其他辅助功能可以根据项目需求逐步扩展
