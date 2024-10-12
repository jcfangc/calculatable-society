mod attribute;
mod derive;

use derive::property_map;
use proc_macro::TokenStream;

/// ## 简介
/// `#[proc_macro_derive(PropertyMap)]` 是一个自定义的过程宏，用于为枚举类型生成一个静态哈希映射表 (`HashMap`) 的方法实现。该映射表将枚举的每个变体映射到相应的 `PropertyConst` 实例，并提供一个 `hash_map()` 静态方法用于访问该映射表。生成的 `hash_map()` 方法返回一个静态 `Lazy<HashMap<EnumType, PropertyConst>>`，确保整个映射在程序生命周期中只初始化一次。
///
/// ## 作用
/// - 为枚举类型自动生成静态映射表，将枚举的每个变体映射到一个常量值。
/// - 可以通过 `EnumType::hash_map()` 方法访问到生成的哈希映射表。
///
/// ## 参数
/// - `input: TokenStream`  
///   表示一个被 `#[derive(PropertyMap)]` 宏标记的目标枚举的完整定义。`TokenStream` 是 Rust 编译器中的一种数据结构，用于表示源代码的原始标记流。`input` 中包含了整个枚举的 AST（抽象语法树），通常通过 `syn` 库解析为 `DeriveInput` 以便进一步处理。
///
/// ## 返回值
/// - `TokenStream`  
///   返回一个 `TokenStream`，其中包含为目标枚举类型生成的 `impl` 块和 `hash_map` 静态方法的代码。该 `TokenStream` 被编译器插入到代码生成阶段，从而为目标枚举类型添加新的功能。
///
/// ## 使用场景
/// - 该宏适用于需要为枚举类型自动生成静态映射的场景，例如将枚举类型映射到常量、配置参数或其他数据结构。
/// - 常见应用包括：
///   1. **枚举到常量的映射表**：自动生成枚举到常量配置的映射，用于序列化、配置管理或定义常量值表。
///   2. **反向查找表**：将枚举值映射到关联数据以实现反向查找，方便在运行时进行快速查找。
///
/// ## 功能描述
/// 该宏会根据枚举的变体生成一个包含映射关系的 `hash_map` 静态方法，其实现逻辑如下：
///
/// - 遍历枚举类型中的所有变体，并为每个变体生成一个 `map.insert` 表达式。
/// - 使用 `once_cell::sync::Lazy` 包装 `HashMap`，确保 `HashMap` 只在第一次调用时初始化。
/// - 返回 `&'static Lazy<HashMap<EnumType, PropertyConst>>`，从而为整个程序提供一个静态的、不可变的哈希映射表。
///
/// ## 如何使用
/// 在目标枚举上直接使用 `#[derive(PropertyMap)]` 即可。例如：
///
/// ```rust
/// #[derive(PropertyMap)]
/// pub enum Property {
///     Flammable,
///     Toxic,
///     Reactive,
/// }
/// ```
///
/// 宏将为 `Property` 枚举自动生成 `hash_map` 方法，并且可以通过 `Property::hash_map()` 访问到包含每个变体的 `HashMap` 实例。
///
/// ## 实现逻辑
/// 1. **解析输入的 `TokenStream`**：  
///   使用 `syn::parse_macro_input!` 将 `TokenStream` 转换为 `DeriveInput` 类型，并解析其 `data` 字段，获取枚举名称和所有变体。
///
/// 2. **匹配目标类型**：  
///   检查输入数据是否为 `Data::Enum` 类型。如果目标类型不是枚举类型，则会直接 `panic!` 并返回错误信息。
///
/// 3. **生成 `map.insert` 插入表达式**：  
///   为枚举的每个变体生成 `map.insert` 代码，并将 `enum_name::variant_name` 作为键值，使用 `PropertyConst` 作为映射值。将所有生成的表达式收集到 `quote!` 的插值中。
///
/// 4. **构建 `impl` 块并生成 `hash_map` 方法**：  
///   使用 `quote!` 生成目标枚举类型的 `impl` 块及其 `hash_map()` 方法，并返回该代码的 `TokenStream`。
///
/// ## 错误处理
/// - 如果目标类型不是枚举类型（例如：结构体或联合体），则会直接 `panic!`，并提示错误信息：`"PropertyMap 派生宏只能用于 Property 枚举类型"`。
/// - 如果在生成过程中路径无法解析（例如 `property_const_path` 的类型路径不正确），可能会引发 `path not found` 错误。调用者需要检查所有相关路径的导入是否正确。
///
/// ## 注意事项
/// - 该宏生成的代码依赖于 `once_cell::sync::Lazy` 进行静态初始化，因此在项目中需要确保 `once_cell` 已被正确引入：
///
///   ```toml
///   [dependencies]
///   once_cell = "1.9"
///   ```
///
/// - 生成的 `hash_map` 是一个静态的 `Lazy` 映射，初始化时具有线程安全特性，可以在多线程场景中安全地调用。
///
/// ## 相关链接
/// - [Rust `proc-macro` 官方文档](https://doc.rust-lang.org/proc_macro/)
/// - [once_cell 使用指南](https://docs.rs/once_cell/latest/once_cell/)
/// - [HashMap 集合类型文档](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
#[proc_macro_derive(PropertyMap)]
pub fn property_map_derive(input: TokenStream) -> TokenStream {
    property_map::derive(input)
}

use attribute::property_model;
#[proc_macro_attribute]
pub fn property_model_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    property_model::attribute(attr, item)
}
