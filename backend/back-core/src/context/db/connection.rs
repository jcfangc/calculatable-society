use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;

// 数据库设置和初始化
pub(crate) async fn create_pool(
    idle_timeout: Option<usize>,
    pool_size: Option<usize>,
    acquire_timeout: Option<usize>,
) -> Arc<Pool<Postgres>> {
    dotenv().ok();

    let idle_timeout = idle_timeout.unwrap_or(30);
    let pool_size = pool_size.unwrap_or(10);
    let acquire_timeout = acquire_timeout.unwrap_or(5);

    // 从环境变量获取数据库配置信息
    let db_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let db_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let db_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    // 动态生成 DATABASE_URL
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    println!("Database URL: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(pool_size as u32)
        .acquire_timeout(Duration::from_secs(acquire_timeout as u64))
        .idle_timeout(Duration::from_secs(idle_timeout as u64))
        .connect(&database_url)
        .await
        .expect("无法构建数据库连接池");

    Arc::new(pool)
}