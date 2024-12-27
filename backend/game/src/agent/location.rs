use crate::agent::mover::Mover;
use serde::Serialize;
use std::fmt;

// 代理人位置结构
#[derive(Debug, Serialize)]
pub(crate) struct Location {
    x: usize,
    y: usize,
}

impl Location {
    // 初始化代理人位置
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Location { x, y }
    }

    // 创建并返回移动偏移，支持链式调用
    pub(crate) fn move_by(self, dx: isize, dy: isize) -> Mover {
        Mover::new(self, dx, dy)
    }

    // 返回位置的不可变借用
    pub(crate) fn get_ref(&self) -> &Self {
        self
    }

    // 获取 x 坐标的不可变借用
    pub(crate) fn x(&self) -> usize {
        self.x
    }

    // 获取 y 坐标的不可变借用
    pub(crate) fn y(&self) -> usize {
        self.y
    }
}

// 打印代理人位置
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(_) => write!(f, "Location {{ x: {}, y: {} }}", self.x, self.y),
        }
    }
}
