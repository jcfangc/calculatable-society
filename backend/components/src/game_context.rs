use crate::environment::map_size::MapSize;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;
use uuid::Uuid;

/// 游戏上下文
#[derive(Debug, Default)]
pub struct GameContext {
    map_size: Option<MapSize>,     // 地图大小，可选
    civilization_id: Option<Uuid>, // 文明编号，使用 UUID
    gravity_const: Option<f64>,    // 重力常数
}

impl GameContext {
    /// 创建一个新的上下文
    fn new() -> Self {
        Self {
            map_size: None,
            civilization_id: None,
            gravity_const: None,
        }
    }
}

// ========================================
// 更新游戏上下文相关方法
// ========================================
impl GameContext {
    /// 通用更新方法
    fn update_global<F>(update_fn: F)
    where
        F: FnOnce(&mut GameContext),
    {
        let global_context = GAME_CONTEXT.clone();
        let mut context = global_context.write().expect("未能获取读锁");
        update_fn(&mut context);
    }

    pub fn update_global_map_size(map_size: MapSize) {
        Self::update_global(|context| context.map_size = Some(map_size));
    }

    pub fn update_global_civilization_id(civilization_id: Uuid) {
        Self::update_global(|context| context.civilization_id = Some(civilization_id));
    }

    pub fn update_global_gravity_const(gravity_const: f64) {
        Self::update_global(|context| context.gravity_const = Some(gravity_const));
    }
}

// ========================================
// 设置游戏上下文相关方法
// ========================================
impl GameContext {
    pub fn with_map_size(mut self, map_size: MapSize) -> Self {
        self.map_size = Some(map_size);
        self
    }

    pub fn with_civilization_id(mut self) -> Self {
        self.civilization_id = Some(Uuid::new_v4());
        self
    }

    pub fn with_gravity_const(mut self, gravity_const: Option<f64>) -> Self {
        self.gravity_const = Some(gravity_const.unwrap_or(10.0));
        self
    }
}

// ========================================
// 获取游戏上下文相关方法
// ========================================
impl GameContext {
    fn get_global_field<T, F>(field_accessor: F, field_name: &'static str) -> T
    where
        F: FnOnce(&GameContext) -> Option<T>,
    {
        let context = GAME_CONTEXT
            .read()
            .unwrap_or_else(|e| panic!("{}", GameContextError::ReadLockFailed(e)));

        field_accessor(&context)
            .unwrap_or_else(|| panic!("{}", GameContextError::ContextFieldNotSet(field_name)))
    }

    pub fn get_map_size() -> MapSize {
        Self::get_global_field(|ctx| ctx.map_size.clone(), "map_size")
    }

    pub fn get_civilization_id() -> Uuid {
        Self::get_global_field(|ctx| ctx.civilization_id, "civilization_id")
    }

    pub fn get_gravity_const() -> f64 {
        Self::get_global_field(|ctx| ctx.gravity_const, "gravity_const")
    }
}

/// 全局游戏上下文实例
static GAME_CONTEXT: Lazy<Arc<RwLock<GameContext>>> =
    Lazy::new(|| Arc::new(RwLock::new(GameContext::new())));

#[derive(Debug, Error)]
pub enum GameContextError {
    /// 获取全局上下文的读锁失败
    #[error("Failed to acquire a read lock on the global game context")]
    ReadLockFailed(#[source] PoisonError<RwLockReadGuard<'static, GameContext>>),

    /// 获取全局上下文的写锁失败
    #[error("Failed to acquire a write lock on the global game context")]
    WriteLockFailed(#[source] PoisonError<RwLockWriteGuard<'static, GameContext>>),

    /// 请求的上下文数据尚未设置（例如 map_size 或 civilization_id）
    #[error("Requested context data not available: {0}")]
    ContextFieldNotSet(&'static str),
}
