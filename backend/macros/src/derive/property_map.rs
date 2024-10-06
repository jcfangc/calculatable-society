use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Path};

// pub enum Property {
//     Flammable = 1, // 可燃性
//     Toxic,         // 毒性
//     ...
// }

// impl Property {
//     pub fn hash_map() -> &'static Lazy<HashMap<Property, PropertyConst>> {
//         static INSTANCE: Lazy<HashMap<Property, PropertyConst>> = Lazy::new(|| {
//             let mut map = HashMap::new();
//             map.insert(
//                 Property::Flammable,
//                 PropertyConst::new(Property::Flammable as i32, Property::Flammable as i32),
//             );
//             map.insert(
//                 Property::Toxic,
//                 PropertyConst::new(Property::Toxic as i32, Property::Toxic as i32),
//             );
//
//             ...
//
//             map
//         });
//         &INSTANCE
//     }
// }

/// 为枚举类型派生 `hash_map` 函数的过程宏。
///
/// 此宏会自动为目标枚举生成一个 `hash_map` 函数，该函数返回一个静态的 `Lazy` 哈希映射表，用于将枚举的每个变体映射到一个 `PropertyConst` 实例。生成的 `hash_map` 函数适合用作枚举属性的常量映射表。
///
/// # 使用场景
///
/// 当你需要为一个枚举类型生成一个映射表（`HashMap`），并将每个枚举变体映射到其相关的常量值时，可以使用该过程宏。此宏会自动遍历所有枚举变体，并将其插入到 `HashMap` 中。
///
/// 例如：
///
/// ```rust
/// #[derive(PropertyMap)]
/// pub enum Property {
///     Flammable,  // 可燃性
///     Toxic,      // 毒性
///     Reactive,   // 反应性
///     Corrosive,  // 腐蚀性
/// }
/// ```
///
/// 此派生宏会为 `Property` 枚举生成如下代码：
///
/// ```rust
/// impl Property {
///     pub fn hash_map() -> &'static Lazy<HashMap<Property, PropertyConst>> {
///         static INSTANCE: Lazy<HashMap<Property, PropertyConst>> = Lazy::new(|| {
///             let mut map = HashMap::new();
///             map.insert(Property::Flammable, PropertyConst::new(1, 1));
///             map.insert(Property::Toxic, PropertyConst::new(2, 2));
///             map.insert(Property::Reactive, PropertyConst::new(3, 3));
///             map.insert(Property::Corrosive, PropertyConst::new(4, 4));
///             map
///         });
///         &INSTANCE
///     }
/// }
/// ```
///
/// # 参数
///
/// - `input`：输入的 `TokenStream`，表示目标枚举的 AST（抽象语法树）。该宏会解析输入的 `DeriveInput` 以确定输入类型是否为枚举类型，并生成相应的 `impl` 代码。
///
/// # 返回值
///
/// 生成的 `TokenStream`，其中包含了目标枚举的 `hash_map` 函数实现。此 `TokenStream` 会被 Rust 编译器插入到目标枚举的 `impl` 块中。
///
/// # 限制
///
/// 此过程宏 **只能用于枚举类型**，如果用于其他类型（如结构体），会触发 `panic!`。
///
/// # 依赖项
///
/// - `once_cell`：使用 `Lazy` 实现静态单例的 `HashMap` 初始化。
/// - `syn`：解析输入的 `TokenStream`。
/// - `quote`：生成输出的 `TokenStream`。
///
/// # 示例
///
/// 该过程宏应用于 `Property` 枚举时，会生成一个 `hash_map` 函数，用于映射每个 `Property` 变体到对应的 `PropertyConst` 实例。
///
/// ```rust
/// #[derive(PropertyMap)]
/// pub enum Property {
///     Flammable,
///     Toxic,
/// }
///
/// // 调用生成的 hash_map 函数
/// let map = Property::hash_map();
/// assert_eq!(map.get(&Property::Flammable), Some(&PropertyConst::new(1, 1)));
/// ```
pub fn derive(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let input = parse_macro_input!(input as DeriveInput);
    // eprintln!("{:#?}", input);

    // 动态生成 PropertyConst 的类型路径
    let property_const_path: Path = parse_quote!(
        types::components_types::shared::resource::property::property_const::PropertyConst
    );
    // eprint!("{:#?}", property_const_path);

    // 获取枚举名称
    let enum_name = &input.ident;

    // 确定输入的数据类型是枚举
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("PropertyMap 派生宏只能用于 Property 枚举类型。"),
    };

    // 生成插入 HashMap 的代码
    let insertions = data_enum.variants.iter().map(|variant| {
        let varient_name = &variant.ident;
        quote! {
            // map.insert(
            //     Property::Flammable,
            //     PropertyConst::new(Property::Flammable as i32, Property::Flammable as i32),
            // );
            map.insert(
                #enum_name::#varient_name,
                #property_const_path::new(
                    #enum_name::#varient_name as i32,
                    #enum_name::#varient_name as i32
                ),
            ) // ) 注意此处的分号会在扩展代码中自动添加 e.g. #(#insertions;)*
        }
    });

    // 生成扩展代码
    let expanded = quote! {
        use once_cell::sync::Lazy;
        use std::collections::HashMap;
        use #property_const_path;

        impl #enum_name {
            pub fn hash_map() -> &'static Lazy<HashMap<#enum_name, #property_const_path>> {
                static INSTANCE: Lazy<HashMap<#enum_name, #property_const_path>> = Lazy::new(|| {
                    let mut map = HashMap::new();
                    #(#insertions;)*
                    map
                });
                &INSTANCE
            }
        }
    };

    TokenStream::from(expanded)
}
