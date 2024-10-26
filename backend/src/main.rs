use context::{AppContext, GLOBAL_APP_CONTEXT};
use log::init_logging;
use tracing;

#[tokio::main]
async fn main() {
    tracing::info!("ESS启动！");

    // 初始化日志，并确保 _guard 保持作用范围，防止程序结束时日志未写入
    let _guard = init_logging();

    // 初始化全局的 AppContext 实例
    let app_context = AppContext::new()
        .with_db_pool(Some(30), Some(10), Some(5))
        .await;

    GLOBAL_APP_CONTEXT
        .set(app_context)
        .expect("应用全局上下文初始化失败");
}
