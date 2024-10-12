use app_context::AppContext;
use core::panic;
use proc_macro::TokenStream;
use std::fs::read_to_string;
use std::path::Path;
use syn::{parse_macro_input, Expr, ExprLit, ItemStruct, Lit, Meta, MetaNameValue};

pub fn attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入
    let input_struct = parse_macro_input!(item as ItemStruct);
    eprintln!("{:#?}", input_struct);

    // 解析属性
    let meta = parse_macro_input!(attr as Meta);
    eprintln!("{:#?}", meta);

    // 获取项目根目录
    let project_root = AppContext::to_map()
        .get(&AppContext::ProjectRoot)
        .unwrap_or_else(|| {
            eprintln!("错误: 无法从 AppContext 获取项目根目录。");
            panic!("未找到项目根目录，程序中止。");
        });

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
    let absolute_path = Path::new(project_root).join(relative_path);
    eprintln!("绝对路径: {:?}", absolute_path);

    // 读取文件内容
    let file_content = read_to_string(&absolute_path).unwrap_or_else(|_| {
        eprintln!("错误: 无法读取文件内容。");
        panic!("无法读取文件内容，程序中止。");
    });

    // 文件内容解析为 TokenStream
    let enum_tokenstream = file_content.parse::<TokenStream>().unwrap_or_else(|_| {
        eprintln!("错误: 无法解析文件内容。");
        panic!("无法解析文件内容，程序中止。");
    });
    eprintln!("{:#?}", enum_tokenstream);

    TokenStream::new()
}
