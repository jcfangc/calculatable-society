pub const COMPONENTS: &str = "components";
pub const CONTROLLERS: &str = "controllers";
pub const EVENTS: &str = "events";
pub const MIDDLEWARES: &str = "middlewares";
pub const MODELS: &str = "models";
pub const REPOSITORIES: &str = "repositories";
pub const SERVICES: &str = "services";
pub const UTILS: &str = "utils";

pub const ALL_MODULES: [&str; 8] = [
    COMPONENTS,
    CONTROLLERS,
    EVENTS,
    MIDDLEWARES,
    MODELS,
    REPOSITORIES,
    SERVICES,
    UTILS,
];

// 使用宏生成模块常量和数组
generate_modules!(
    "components",
    "controllers",
    "events",
    "middlewares",
    "models",
    "repositories",
    "services",
    "utils",
    "custom_module_with_underscore"
);
