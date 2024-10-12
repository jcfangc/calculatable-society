use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

// 数据库设置和初始化
pub async fn create_pool(
    idle_timeout: Option<u64>, // 使用 Option 包装可选参数
    pool_size: Option<u32>,
) -> Pool<Postgres> {
    dotenv().ok();
    // 设置默认值
    let idle_timeout = idle_timeout.unwrap_or(30);
    let pool_size = pool_size.unwrap_or(10);
    // 从 .env 中获取数据库连接字符串和其他参数
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(pool_size)
        .idle_timeout(Some(Duration::from_secs(idle_timeout)))
        .connect(&database_url)
        .await
        .expect("Failed to create pool.")
}
