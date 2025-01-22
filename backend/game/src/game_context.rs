use crate::environment::cartesian_vec_2d::CartesianVec2D;
use crate::environment::map_size::MapSize;
use my_proc_macro::Literal;
use once_cell::sync::Lazy;
use share_and_commute::errors::context_error::ContextError;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

pub type GameContextError = ContextError<GameContext>;

/// 全局游戏上下文实例
static GAME_CONTEXT: Lazy<Arc<RwLock<GameContext>>> =
    Lazy::new(|| Arc::new(RwLock::new(GameContext::new())));

/// 游戏上下文
#[derive(Debug, Default, Literal)]
pub struct GameContext {
    /// 地图大小
    map_size: Option<MapSize>,
    /// 文明编号，使用 UUID
    civilization_id: Option<Uuid>,
    /// 重力常数
    gravity_const: Option<f64>,
    /// 六边形地图基向量 x 在笛卡尔空间投影
    #[notLiteral]
    x_base_vector: CartesianVec2D,
    /// 六边形地图基向量 y 在笛卡尔空间投影
    #[notLiteral]
    y_base_vector: CartesianVec2D,
}

impl GameContext {
    /// 创建一个新的上下文
    fn new() -> Self {
        let x_base_vector = CartesianVec2D::new(0.5, (3.0f64.sqrt()) * 0.5);
        let y_base_vector = CartesianVec2D::new(1.0, 0.0);

        Self {
            map_size: None,
            civilization_id: None,
            gravity_const: None,
            x_base_vector,
            y_base_vector,
        }
    }
}

// update
impl GameContext {
    /// 通用更新方法
    fn update_game<F>(update_fn: F)
    where
        F: FnOnce(&mut GameContext),
    {
        let game_context = GAME_CONTEXT.clone();
        let mut context = game_context
            .write()
            .expect("未能获取读锁，更新游戏上下文失败");
        update_fn(&mut context);
    }

    pub fn update_game_map_size(map_size: MapSize) {
        Self::update_game(|context| context.map_size = Some(map_size));
    }

    pub fn update_game_civilization_id(civilization_id: Uuid) {
        Self::update_game(|context| context.civilization_id = Some(civilization_id));
    }

    pub fn update_game_gravity_const(gravity_const: f64) {
        Self::update_game(|context| context.gravity_const = Some(gravity_const));
    }
}

// with
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

// get
impl GameContext {
    /// 访问全局上下文并执行指定操作。
    ///
    /// ### 参数
    /// * `accessor` - 操作上下文的闭包函数。
    ///
    /// ### 返回值
    /// 返回闭包执行结果。
    fn access_game_context<F, T>(accessor: F) -> T
    where
        F: FnOnce(&GameContext) -> T,
    {
        let context = GAME_CONTEXT
            .read()
            .unwrap_or_else(|e| panic!("{}", GameContextError::ReadLockFailed(e)));

        accessor(&context)
    }

    /// 获取全局可选字段值，如果字段未设置则触发 panic。
    ///
    /// ### 参数
    /// * `field_accessor` - 获取字段的闭包函数。
    /// * `field_name` - 字段名称，用于错误提示。
    ///
    /// ### 返回值
    /// 返回字段值。
    fn get_optional_field<T, F>(field_accessor: F, field_name: &'static str) -> T
    where
        F: FnOnce(&GameContext) -> Option<T>,
    {
        Self::access_game_context(|context| {
            field_accessor(context)
                .unwrap_or_else(|| panic!("{}", GameContextError::ContextFieldNotSet(field_name)))
        })
    }

    /// 获取全局字段值。
    ///
    /// ### 参数
    /// * `field_accessor` - 获取字段的闭包函数。
    ///
    /// ### 返回值
    /// 返回字段值。
    fn get_field<T, F>(field_accessor: F) -> T
    where
        F: FnOnce(&GameContext) -> T,
    {
        Self::access_game_context(field_accessor)
    }

    /// 获取地图大小。
    ///
    /// ### 返回值
    /// 返回地图大小的 `MapSize` 对象。
    pub fn get_map_size() -> MapSize {
        Self::get_optional_field(|ctx| ctx.map_size.clone(), GameContext::MAP_SIZE)
    }

    /// 获取文明 ID。
    ///
    /// ### 返回值
    /// 返回文明的 `Uuid`。
    pub fn get_civilization_id() -> Uuid {
        Self::get_optional_field(|ctx| ctx.civilization_id, GameContext::CIVILIZATION_ID)
    }

    /// 获取重力常数。
    ///
    /// ### 返回值
    /// 返回重力常数的值。
    pub fn get_gravity_const() -> f64 {
        Self::get_optional_field(|ctx| ctx.gravity_const, GameContext::GRAVITY_CONST)
    }

    /// 获取 X 基向量。
    ///
    /// ### 返回值
    /// 返回 X 基向量的 `CartesianCoord` 对象。
    pub fn get_x_base_vector() -> CartesianVec2D {
        Self::get_field(|ctx| ctx.x_base_vector)
    }

    /// 获取 Y 基向量。
    ///
    /// ### 返回值
    /// 返回 Y 基向量的 `CartesianCoord` 对象。
    pub fn get_y_base_vector() -> CartesianVec2D {
        Self::get_field(|ctx| ctx.y_base_vector)
    }
}
