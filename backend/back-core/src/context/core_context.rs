use crate::context::db::connection::create_pool;
use crate::context::redis::connection::init_redis_connection;
use my_proc_macro::Literal;
use once_cell::sync::Lazy;
use redis::aio::Connection as RedisConnection;
use redis::AsyncCommands;
use redis::Client as RedisClient;
use share_and_commute::errors::context_error::ContextError;
use sqlx::{Pool, Postgres};
use std::sync::{Arc, RwLock};

pub type AppContextError = ContextError<AppContext>;

/// 全局 AppContext 实例
static APP_CONTEXT: Lazy<Arc<RwLock<AppContext>>> =
    Lazy::new(|| Arc::new(RwLock::new(AppContext::new())));

/// AppContext 结构体，内部存储需要的应用上下文数据
#[derive(Debug, Default, Literal)]
pub struct AppContext {
    /// 数据库连接池，可选字段，如果尚未初始化则为 None
    db_pool: Option<Arc<Pool<Postgres>>>,

    /// Redis 连接客户端，可选字段
    redis_client: Option<Arc<RedisClient>>,
}

impl AppContext {
    /// 创建一个新的上下文
    pub fn new() -> Self {
        Self {
            db_pool: None,
            redis_client: None,
        }
    }
}

/// with
impl AppContext {
    /// 使用数据库连接池
    pub async fn with_db_pool(
        mut self,
        idle_timeout: Option<usize>,
        pool_size: Option<usize>,
        acquire_timeout: Option<usize>,
    ) -> Self {
        self.db_pool = Some(create_pool(idle_timeout, pool_size, acquire_timeout).await);
        self
    }

    /// 使用 Redis 连接客户端
    pub async fn with_redis_client(mut self) -> Self {
        self.redis_client = Some(init_redis_connection(None, None, None).await);
        self
    }
}

/// update
impl AppContext {
    /// 通用的全局更新方法：获取写锁，执行闭包更新。
    fn update_global<F>(update_fn: F)
    where
        F: FnOnce(&mut AppContext),
    {
        let global_context = APP_CONTEXT.clone();
        let mut context = global_context
            .write()
            .expect("未能获取读锁，更新应用上下文失败");
        update_fn(&mut context);
    }

    /// 初始化（或更新）全局数据库连接池。
    pub async fn update_global_db_pool(
        idle_timeout: Option<usize>,
        pool_size: Option<usize>,
        acquire_timeout: Option<usize>,
    ) {
        let pool = create_pool(idle_timeout, pool_size, acquire_timeout).await;
        Self::update_global(|context| {
            context.db_pool = Some(pool);
        });
    }

    /// 初始化（或更新）全局 Redis 连接客户端。
    pub async fn update_global_redis_client(
        host: Option<&str>,
        port: Option<&str>,
        password: Option<&str>,
    ) {
        let client = init_redis_connection(host, port, password).await;
        Self::update_global(|context| {
            context.redis_client = Some(client);
        });
    }
}

/// get
impl AppContext {
    /// 通用的全局只读访问方法：获取读锁并执行闭包。
    fn access_app_context<F, T>(accessor: F) -> T
    where
        F: FnOnce(&AppContext) -> T,
    {
        let context = APP_CONTEXT
            .read()
            .unwrap_or_else(|e| panic!("{}", AppContextError::ReadLockFailed(e)));
        accessor(&context)
    }

    /// 获取全局可选字段，若未初始化则触发 panic。
    ///
    /// 可以复用此方法来获取类似 `db_pool` 等可选字段的值，若为空则报错。
    fn get_global_optional_field<T, F>(field_accessor: F, field_name: &'static str) -> T
    where
        F: FnOnce(&AppContext) -> Option<T>,
    {
        Self::access_app_context(|context| {
            field_accessor(context)
                .unwrap_or_else(|| panic!("{}", AppContextError::ContextFieldNotSet(field_name)))
        })
    }

    /// 获取全局数据库连接池，如果尚未初始化则会 panic。
    pub fn get_db_pool() -> Arc<Pool<Postgres>> {
        Self::get_global_optional_field(|ctx| ctx.db_pool.clone(), AppContext::DB_POOL)
    }

    /// 获取全局 Redis 连接客户端，如果尚未初始化则会 panic。
    pub fn get_redis_client() -> Arc<RedisClient> {
        Self::get_global_optional_field(|ctx| ctx.redis_client.clone(), AppContext::REDIS_CLIENT)
    }
}
