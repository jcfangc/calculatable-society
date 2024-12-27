use crate::agent::location::Location;
use crate::environment::landscape::Landscape;
use serde::Serialize;
use std::fmt;

// 移动结构，表示一个包含偏移量的移动操作
#[derive(Debug, Serialize)]
pub(crate) struct Mover {
    location: Location,
    dx: isize,
    dy: isize,
}

impl Mover {
    pub(crate) fn new(location: Location, dx: isize, dy: isize) -> Self {
        Mover { location, dx, dy }
    }

    // 应用移动到指定地图，并返回更新后的位置，支持链式调用
    pub(crate) fn on(mut self, map: &Landscape) -> Location {
        let new_x =
            (self.location.x() as isize + self.dx).rem_euclid(map.map_size().width() as isize);
        let new_y =
            (self.location.y() as isize + self.dy).rem_euclid(map.map_size().height() as isize);

        self.location = Location::new(new_x as usize, new_y as usize);

        self.location // 返回更新后的 Location
    }

    // 不可变借用访问方法
    pub(crate) fn get_ref(&self) -> &Self {
        self
    }

    pub(crate) fn location(&self) -> &Location {
        self.location.get_ref()
    }

    pub(crate) fn dx(&self) -> isize {
        self.dx
    }

    pub(crate) fn dy(&self) -> isize {
        self.dy
    }
}

// 打印 Mover 的状态
impl fmt::Display for Mover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(_) => write!(
                f,
                "Mover {{ location: {}, dx: {}, dy: {} }}",
                self.location, self.dx, self.dy
            ),
        }
    }
}
