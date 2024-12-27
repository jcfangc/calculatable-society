use crate::context::db::connection::create_pool;
use crate::context::db::db_contexted::DatabaseContexted;
use once_cell::sync::Lazy;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::OnceCell;

// 全局的 AppContext 实例，使用 Lazy 包装
pub static GLOBAL_APP_CONTEXT: Lazy<OnceCell<AppContext>> = Lazy::new(|| OnceCell::new());

// 定义 AppContext 结构体
#[derive(Debug)]
pub struct AppContext {
    pub db_pool: OnceCell<Arc<Pool<Postgres>>>, // 数据库连接池
}

impl AppContext {
    // 构造函数，初始化 OnceCell
    pub fn new() -> Self {
        AppContext {
            db_pool: OnceCell::new(),
        }
    }

    // 带参数的 with_db_pool 方法，用于初始化并返回修改后的 AppContext 实例
    pub async fn with_db_pool(
        self,
        idle_timeout: Option<usize>,
        pool_size: Option<usize>,
        acquire_timeout: Option<usize>,
    ) -> Self {
        let pool = create_pool(idle_timeout, pool_size, acquire_timeout).await;
        self.db_pool.get_or_init(|| async { pool }).await;

        self // 返回当前的 AppContext 实例，实现链式调用
    }
}

// 为 AppContext 实现 DatabaseContext 特质
#[async_trait::async_trait]
impl DatabaseContexted for AppContext {
    async fn db_pool(&self) -> &Arc<Pool<Postgres>> {
        self.db_pool.get().expect("数据库连接池未初始化")
    }
}
