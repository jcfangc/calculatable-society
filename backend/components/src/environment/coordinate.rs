use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::hexagon::t_hexa_distanced::HexaDistanced;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use crate::environment::t_indexed::Indexed;
use crate::game_context::GameContext;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize)]
pub(crate) struct Coordinate {
    /// 行坐标
    y: usize,
    /// 列坐标
    x: usize,
}

impl Coordinate {
    /// 创建一个新的坐标
    pub(crate) fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    /// 计算几何关系对应的坐标映射
    /// 该方法会根据传入的关系类型 `R` 返回一个包含各个方向对应坐标的映射表。
    pub(crate) fn get_relations_map<R>(&self) -> HashMap<R, Self>
    where
        R: HexaRelational,
    {
        // 获取地图的宽高信息
        let (height, width) = GameContext::get_map_size().as_tuple();

        // 遍历方向与偏移量的映射，计算每个方向对应的新坐标
        R::from_relation_to_coordinate_shift()
            .iter()
            .map(|(&relation, coordinate_shift)| {
                // 计算新的 y 和 x 坐标，考虑宽高的环绕（模数）效果
                let new_y =
                    (self.y as isize + coordinate_shift.dy()).rem_euclid(height as isize) as usize;
                let new_x =
                    (self.x as isize + coordinate_shift.dx()).rem_euclid(width as isize) as usize;

                // 返回方向与新的坐标
                (relation, Self::new(new_y, new_x))
            })
            .collect()
    }

    /// 缩放坐标
    pub(crate) fn scale(&self, scale: usize) -> Self {
        Self::new(self.y * scale, self.x * scale)
    }

    /// 带环绕效果的加法运算
    fn add_wrapping(self, other: Self) -> Self {
        let (height, width) = GameContext::get_map_size().as_tuple();
        Self {
            y: (self.y + other.y) % height,
            x: (self.x + other.x) % width,
        }
    }

    /// 带环绕效果的减法运算
    fn sub_wrapping(self, other: Self) -> Self {
        let (height, width) = GameContext::get_map_size().as_tuple();
        Self {
            y: ((self.y as isize - other.y as isize).rem_euclid(height as isize)) as usize,
            x: ((self.x as isize - other.x as isize).rem_euclid(width as isize)) as usize,
        }
    }

    /// 带环绕效果的乘法运算
    fn mul_wrapping(self, scalar: usize) -> Self {
        let (height, width) = GameContext::get_map_size().as_tuple();
        Self {
            y: (self.y * scalar) % height,
            x: (self.x * scalar) % width,
        }
    }
}

impl HexaDistanced for Coordinate {
    /// 立方体坐标下的隐藏第三轴
    fn z(&self) -> isize {
        -(self.x as isize + self.y as isize)
    }

    /// 计算两坐标之间的六边形距离
    fn distance_to(&self, other: &Coordinate) -> usize {
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        let dz = (self.z() - other.z()).abs();
        ((dx + dy + dz) / 2) as usize
    }
}

impl Add<Self> for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add_wrapping(other)
    }
}

impl Add<CoordinateShift> for Coordinate {
    type Output = Self;

    fn add(self, shift: CoordinateShift) -> Self {
        let (height, width) = GameContext::get_map_size().as_tuple();
        Self {
            y: ((self.y as isize + shift.dy()).rem_euclid(height as isize)) as usize,
            x: ((self.x as isize + shift.dx()).rem_euclid(width as isize)) as usize,
        }
    }
}

impl Add<Coordinate> for CoordinateShift {
    type Output = Coordinate;

    fn add(self, coord: Coordinate) -> Coordinate {
        let (height, width) = GameContext::get_map_size().as_tuple();
        Coordinate {
            y: ((coord.y as isize + self.dy()).rem_euclid(height as isize)) as usize,
            x: ((coord.x as isize + self.dx()).rem_euclid(width as isize)) as usize,
        }
    }
}

impl Sub<Self> for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub_wrapping(other)
    }
}

impl Mul<usize> for Coordinate {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        self.mul_wrapping(scalar)
    }
}

impl Indexed for Coordinate {
    fn y(&self) -> usize {
        self.y
    }

    fn x(&self) -> usize {
        self.x
    }
}
