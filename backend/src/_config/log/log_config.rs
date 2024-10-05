use crate::_config::consts::const_modules;
use crate::_config::consts::const_routes::LOG_DIR;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

// 定义一个宏来创建并添加所有日志层
macro_rules! create_and_register_log_layers {
    ($registry:expr, $($module_name:expr),*) => {{
        $registry
        $(.with(
            fmt::layer()
                .with_writer(|| {
                    std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(format!("{}/{}.log", LOG_DIR, $module_name))
                        .unwrap()
                })
                .with_filter(filter_fn(|metadata| {
                    metadata.module_path().unwrap_or("") == $module_name
                })),
        ))*
    }};
}

pub fn init_logging() {
    // 使用宏创建所有模块的日志层
    let subscriber = create_and_register_log_layers!(
        Registry::default(),
        const_modules::REPOSITORIES,
        const_modules::EVENTS,
        const_modules::SERVICES,
        const_modules::COMPONENTS,
        const_modules::MODELS,
        const_modules::CONTROLLERS,
        const_modules::MIDDLEWARES,
        const_modules::UTILS
    )
    .with(fmt::layer().with_writer(|| {
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}/default.log", LOG_DIR))
            .unwrap()
    }));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}
