use crate::environment::map_size::MapSize;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// 游戏上下文
#[derive(Debug, Default)]
pub struct GameContext {
    map_size: Option<MapSize>, // 地图大小，可选
    civilization_id: Option<Uuid>, // 文明编号，使用 UUID
                               // 其他可选字段...
}

impl GameContext {
    /// 创建一个新的上下文
    fn new() -> Self {
        Self {
            map_size: None,
            civilization_id: None,
        }
    }

    /// 更新全局上下文中的地图大小
    pub fn update_global_map_size(map_size: MapSize) {
        let global_context = GAME_CONTEXT.clone();
        let mut context = global_context
            .write()
            .expect("Failed to acquire write lock");

        // 更新 map_size
        context.map_size = Some(map_size);
    }

    /// 设置文明编号到全局上下文
    pub fn update_global_civilization_id(civilization_id: Uuid) {
        let global_context = GAME_CONTEXT.clone();
        let mut context = global_context
            .write()
            .expect("Failed to acquire write lock");

        // 更新文明编号
        context.civilization_id = Some(civilization_id);
    }

    /// 设置地图大小
    pub fn with_map_size(mut self, map_size: MapSize) -> Self {
        self.map_size = Some(map_size);
        self
    }

    /// 设置文明编号
    pub fn with_civilization_id(mut self, civilization_id: Uuid) -> Self {
        self.civilization_id = Some(civilization_id);
        self
    }

    /// 生成一个新的文明编号并设置
    pub fn generate_new_civilization_id(mut self) -> Self {
        self.civilization_id = Some(Uuid::new_v4());
        self
    }

    /// 获取地图大小
    pub fn map_size(&self) -> Option<MapSize> {
        self.map_size.clone()
    }

    /// 获取文明编号
    pub fn civilization_id(&self) -> Option<Uuid> {
        self.civilization_id
    }

    /// 获取全局实例的只读引用
    pub fn global_instance() -> Arc<RwLock<GameContext>> {
        GAME_CONTEXT.clone()
    }
}

/// 全局游戏上下文实例
pub static GAME_CONTEXT: Lazy<Arc<RwLock<GameContext>>> =
    Lazy::new(|| Arc::new(RwLock::new(GameContext::new())));
