use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

/// 通用上下文错误类型
#[derive(Debug, Error)]
pub enum ContextError<T>
where
    T: 'static,
{
    /// 获取全局上下文的读锁失败
    #[error("获取全局上下文的读锁失败")]
    ReadLockFailed(PoisonError<RwLockReadGuard<'static, T>>),

    /// 获取全局上下文的写锁失败
    #[error("获取全局上下文的写锁失败")]
    WriteLockFailed(PoisonError<RwLockWriteGuard<'static, T>>),

    /// 请求的上下文字段尚未被初始化
    #[error("请求的上下文字段尚未被初始化")]
    ContextFieldNotSet(&'static str),
}
