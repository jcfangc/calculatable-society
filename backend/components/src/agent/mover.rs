use crate::agent::location::Location;
use crate::environment::toroidal_map::ToroidalMap;
use std::fmt;

// 移动结构，表示一个包含偏移量的移动操作
#[derive(Debug)]
pub struct Mover {
    location: Location,
    dx: isize,
    dy: isize,
}

impl Mover {
    pub fn new(location: Location, dx: isize, dy: isize) -> Self {
        Mover { location, dx, dy }
    }

    // 应用移动到指定地图，并返回更新后的位置，支持链式调用
    pub fn on(mut self, map: &ToroidalMap) -> Location {
        let new_x = (self.location.x() as isize + self.dx).rem_euclid(map.width() as isize);
        let new_y = (self.location.y() as isize + self.dy).rem_euclid(map.height() as isize);

        self.location = Location::new(new_x as usize, new_y as usize);

        self.location // 返回更新后的 Location
    }

    // 不可变借用访问方法
    pub fn get_ref(&self) -> &Self {
        self
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn dx(&self) -> isize {
        self.dx
    }

    pub fn dy(&self) -> isize {
        self.dy
    }
}

// 打印 Mover 的状态
impl fmt::Display for Mover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "移动器位于位置({}, {})，移动偏移量为 dx: {}，dy: {}",
            self.location.x(),
            self.location.y(),
            self.dx,
            self.dy
        )
    }
}
