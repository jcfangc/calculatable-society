use database::database_config::create_pool;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::env::var;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub project_root: Option<String>,
    pub database_pool: Option<Arc<Pool<Postgres>>>,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            project_root: None,
            database_pool: None,
        }
    }

    pub fn with_project_root(&mut self) -> &mut Self {
        // 如果已经设置了项目根目录，则直接返回
        if self.project_root.is_some() {
            return self;
        }

        // 从环境变量中获取项目根目录
        dotenv().ok();
        self.project_root = Some(var("PROJECT_ROOT").expect("PROJECT_ROOT 必须设置。"));
        self
    }

    pub async fn with_database_pool(&mut self) -> &mut Self {
        // 如果已经设置了数据库连接池，则直接返回
        if self.database_pool.is_some() {
            return self;
        }

        // 创建数据库连接池
        self.database_pool = Some(create_pool(Some(30), Some(15), Some(5)).await);
        self
    }

    pub fn get_instance() -> &'static Mutex<AppContext> {
        &APP_CONTEXT
    }

    pub fn update(&mut self) -> &mut Self {
        let mut app_context = APP_CONTEXT.lock().unwrap();
        *app_context = self.clone(); // 通过 clone() 复制到全局的 APP_CONTEXT
        self // 返回 &mut self 以支持链式调用
    }
}

// 使用 Lazy 来创建全局的 AppContext 单例
static APP_CONTEXT: Lazy<Mutex<AppContext>> = Lazy::new(|| Mutex::new(AppContext::new()));
