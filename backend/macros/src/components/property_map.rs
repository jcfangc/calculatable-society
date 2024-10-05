use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub enum Property {
//     Flammable = 1, // 可燃性
//     Toxic,         // 毒性
//     Reactive,      // 反应性
//     Corrosive,     // 腐蚀性
//     Oxidizer,      // 氧化性
//     AcidBase,      // 酸碱性
//     Phase,         // 相态
//     Conductive,    // 导电性
//     Magnetic,      // 磁性
//     Brittle,       // 脆性
//     Malleable,     // 延展性
//     Elastic,       // 弹性
//     Transparent,   // 透明性
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
//             map.insert(
//                 Property::Reactive,
//                 PropertyConst::new(Property::Reactive as i32, Property::Reactive as i32),
//             );
//             map.insert(
//                 Property::Corrosive,
//                 PropertyConst::new(Property::Corrosive as i32, Property::Corrosive as i32),
//             );
//             map.insert(
//                 Property::Oxidizer,
//                 PropertyConst::new(Property::Oxidizer as i32, Property::Oxidizer as i32),
//             );
//             map.insert(
//                 Property::AcidBase,
//                 PropertyConst::new(Property::AcidBase as i32, Property::AcidBase as i32),
//             );
//             map.insert(
//                 Property::Phase,
//                 PropertyConst::new(Property::Phase as i32, Property::Phase as i32),
//             );
//             map.insert(
//                 Property::Conductive,
//                 PropertyConst::new(Property::Conductive as i32, Property::Conductive as i32),
//             );
//             map.insert(
//                 Property::Magnetic,
//                 PropertyConst::new(Property::Magnetic as i32, Property::Magnetic as i32),
//             );
//             map.insert(
//                 Property::Brittle,
//                 PropertyConst::new(Property::Brittle as i32, Property::Brittle as i32),
//             );
//             map.insert(
//                 Property::Malleable,
//                 PropertyConst::new(Property::Malleable as i32, Property::Malleable as i32),
//             );
//             map.insert(
//                 Property::Elastic,
//                 PropertyConst::new(Property::Elastic as i32, Property::Elastic as i32),
//             );
//             map.insert(
//                 Property::Transparent,
//                 PropertyConst::new(Property::Transparent as i32, Property::Transparent as i32),
//             );
//             map
//         });
//         &INSTANCE
//     }
// }

pub fn property_map_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    // 检查输入是否是一个枚举
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("PropertyMap can only be derived for enums."),
    };

    // 生成 `PropertyConst` 的哈希表项
    let hash_map_entries = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            map.insert(
                #enum_name::#variant_name,
                PropertyConst::new(#enum_name::#variant_name as i32, #enum_name::#variant_name as i32),
            );
        }
    });

    // 生成 `hash_map` 函数的实现
    let expanded = quote! {
        impl #enum_name {
            pub fn hash_map() -> &'static std::lazy::Lazy<std::collections::HashMap<#enum_name, PropertyConst>> {
                static INSTANCE: std::lazy::Lazy<std::collections::HashMap<#enum_name, PropertyConst>> = std::lazy::Lazy::new(|| {
                    let mut map = std::collections::HashMap::new();
                    #(#hash_map_entries)*
                    map
                });
                &INSTANCE
            }
        }
    };

    TokenStream::from(expanded)
}
