use app_context::AppContext;
use core::panic;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::fs::read_to_string;
use std::path::Path;
use syn::{
    parse_macro_input, parse_str, Expr, ExprLit, Fields, File, Item, ItemStruct, Lit, Meta,
    MetaNameValue,
};

pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident;
    let new_struct_name = syn::Ident::new(
        &format!("{}Model", struct_name.to_string()),
        struct_name.span(),
    );
    // 提取字段，移除大括号
    let struct_fields = match &input_struct.fields {
        Fields::Named(named_fields) => {
            let fields = named_fields.named.iter().map(|field| field);
            quote! { #(#fields,)* } // 移除大括号
        }
        _ => quote! {},
    };

    // 解析属性
    let meta = parse_macro_input!(attr as Meta);

    // 获取项目根目录
    let project_root = AppContext::get_instance()
        .lock()
        .unwrap()
        .with_project_root() // 设置项目根目录
        .update() // 更新单例
        .project_root // 获取项目根目录
        .clone() // 克隆项目根目录
        .unwrap(); // 解包 Option<String>

    // 获取传入的相对路径
    let relative_path = if let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(path_str),
            ..
        }),
        ..
    }) = meta
    {
        // 提取路径字符串的值
        path_str.value()
    } else {
        // 如果属性格式不正确，抛出错误
        panic!("请输入 Property 枚举类型的相对路径！");
    };

    // 拼接绝对路径
    let absolute_path = Path::new(&project_root).join(relative_path);

    let file_content = read_to_string(&absolute_path).expect("无法读取文件");

    // 使用 syn::File 直接解析文件内容为 AST
    let syntax_tree: File = parse_str(&file_content).unwrap_or_else(|_| {
        eprintln!("错误: 无法解析文件内容为语法树。");
        panic!("无法解析文件内容，程序中止。");
    });

    // 提取枚举变体并生成字段
    let enum_variants_fields: Vec<TokenStream2> = syntax_tree
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Enum(item_enum) = item {
                Some(item_enum.variants.iter().map(|variant| {
                    let field_name = variant.ident.to_string().to_lowercase(); // 变体名作为字段名
                    let field_ident = syn::Ident::new(&field_name, variant.ident.span());
                    quote! {
                        pub #field_ident: f64, // 假设每个字段是 f64 类型
                    }
                }))
            } else {
                None
            }
        })
        .flatten() // 将多个迭代器平铺为一个
        .collect(); // 收集到 Vec<TokenStream2>

    let expanded = quote! {
        use sqlx::FromRow;

        #[derive(Debug, FromRow)]
        pub struct #new_struct_name {
            #struct_fields
            #(#enum_variants_fields)*
        }
    };

    // 生成属性代码
    TokenStream::from(expanded)
}
