use decla_macro::enum_map;
use std::env::var;

enum_map! {
    pub AppContext => String {
        ProjectRoot => var("CARGO_MANIFEST_DIR").unwrap(),
    }
}

#[test]
fn test_app_context() {
    println!("{:?}", AppContext::to_map());
}
