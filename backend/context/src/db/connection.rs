use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;

// 数据库设置和初始化
pub(crate) async fn create_pool(
    idle_timeout: Option<usize>, // 使用 Option 包装可选参数
    pool_size: Option<usize>,
    acquire_timeout: Option<usize>,
) -> Arc<Pool<Postgres>> {
    dotenv().ok();
    // 设置默认值
    let idle_timeout = idle_timeout.unwrap_or(30);
    let pool_size = pool_size.unwrap_or(10);
    let acquire_timeout = acquire_timeout.unwrap_or(5);
    // 从 .env 中获取数据库连接字符串和其他参数
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(pool_size as u32)
        .acquire_timeout(Duration::from_secs(acquire_timeout as u64)) // 获取连接的超时时间
        .idle_timeout(Duration::from_secs(idle_timeout as u64))
        .connect(&database_url)
        .await
        .expect("无法构建数据库连接池");

    Arc::new(pool) // 使用 Arc 包装 Pool 以确保线程安全
}
