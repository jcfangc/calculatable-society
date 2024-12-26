// use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ContextError {
    // #[error("数据库连接池初始化失败")]
    // DBInitError(#[from] SqlxError),

    // #[error("数据库连接池未初始化")]
    // DBNotInitialized,
}
