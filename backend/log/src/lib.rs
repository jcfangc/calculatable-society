use dotenvy::dotenv;
use std::{env, fs, path::Path};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

/// Initializes logging with JSON file output and compact terminal output.
pub fn init_logging() -> WorkerGuard {
    dotenv().ok();
    let log_dir = setup_log_dir();

    let (non_blocking, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily(&log_dir, "app.json"));

    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .without_time()
        .with_ansi(true)
        .compact()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")))
        .with(file_layer)
        .with(console_layer)
        .init();

    guard
}

/// 获取项目的根目录路径。
///
/// 该路径通过读取环境变量 `PROJECT_ROOT` 来确定。如果环境变量未设置，
/// 则使用默认路径 `/workspaces/EvoScoSim/backend`。
///
/// # 返回值
///
/// 返回一个字符串，表示项目的根目录路径。
fn setup_log_dir() -> String {
    let root_dir =
        env::var("PROJECT_ROOT").unwrap_or_else(|_| "/workspaces/EvoScoSim/backend".to_string());
    let log_dir = Path::new(&root_dir).join("log");
    fs::create_dir_all(&log_dir).expect("日志目录创建失败！");
    log_dir.to_str().unwrap().to_string()
}
