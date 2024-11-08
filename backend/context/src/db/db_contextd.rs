use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;

// 定义 DatabaseContext 特质
#[async_trait::async_trait]
pub trait DatabaseContexted {
    async fn db_pool(&self) -> Arc<Pool<Postgres>>;
}
