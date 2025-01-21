use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

/// 自定义派生宏，用于自动生成结构体字段对应的常量字符串。
///
/// ### 功能
/// - 对结构体的每个字段生成一个 `pub const` 常量，常量名为字段名，值为字段名的字符串形式。
/// - 如果字段上标注了 `#[notLiteral]`，则跳过该字段，不生成对应的常量。
///
/// ### 使用方法
/// - 在结构体定义上添加 `#[derive(Literal)]`。
/// - 如果某些字段不需要生成常量，在字段上添加 `#[notLiteral]` 属性。
///
/// ### 示例
/// ```rust
/// #[derive(Literal)]
/// pub struct Example {
///     pub field1: i32,
///     #[notLiteral]
///     pub field2: String, // 此字段不会生成常量
/// }
///
/// // 自动生成代码
/// impl Example {
///     pub const FIELD1: &'static str = "field1";
/// }
/// ```
#[proc_macro_derive(Literal, attributes(notLiteral))]
pub fn derive_literal(input: TokenStream) -> TokenStream {
    // 将输入的 TokenStream 解析为 DeriveInput（即结构体的定义）。
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体的名称。
    let struct_name = input.ident;

    // 确保派生宏只用于结构体，否则返回编译错误。
    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        return syn::Error::new_spanned(struct_name, "Literal 只能用于结构体的派生")
            .to_compile_error()
            .into();
    };

    // 遍历结构体字段，生成对应的常量。
    let const_declarations = fields.iter().filter_map(|field| {
        // 获取字段名（如果字段没有名字，例如 tuple struct，则跳过）。
        let field_name = field.ident.as_ref()?; // 使用 `?` 操作符优雅处理 `Option`。

        // 将字段名转为字符串形式。
        let field_name_str = field_name.to_string();

        // 检查字段属性，判断是否标注了 `#[notLiteral]`。
        let not_literal = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("notLiteral"));

        // 如果标注了 `#[notLiteral]`，跳过该字段。
        if not_literal {
            return None;
        }

        // 转换字段名为大写的 SNAKE_CASE 格式，并将其转为 `Ident`
        let const_name = Ident::new(&field_name_str.to_case(Case::UpperSnake), field_name.span());

        // 为字段生成 `pub const` 的定义。
        Some(quote! {
            pub const #const_name: &'static str = #field_name_str;
        })
    });

    // 将生成的常量代码注入到结构体的 impl 块中。
    let expanded = quote! {
        impl #struct_name {
            #(#const_declarations)*
        }
    };

    // 将生成的代码转换为 TokenStream 返回给编译器。
    expanded.into()
}
