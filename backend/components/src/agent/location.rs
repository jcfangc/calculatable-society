use crate::agent::mover::Mover;
use std::fmt;

// 代理人位置结构
#[derive(Debug)]
pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    // 初始化代理人位置
    pub fn new(x: usize, y: usize) -> Self {
        Location { x, y }
    }

    // 创建并返回移动偏移，支持链式调用
    pub fn move_by(self, dx: isize, dy: isize) -> Mover {
        Mover::new(self, dx, dy)
    }

    // 返回位置的不可变借用
    pub fn get_ref(&self) -> &Self {
        self
    }

    // 获取 x 坐标的不可变借用
    pub fn x(&self) -> usize {
        self.x
    }

    // 获取 y 坐标的不可变借用
    pub fn y(&self) -> usize {
        self.y
    }
}

// 打印代理人位置
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location({}, {})", self.x, self.y)
    }
}
