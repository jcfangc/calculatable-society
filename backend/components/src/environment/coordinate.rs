use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::hexagon::t_hexa_distanced::HexaDistanced;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use crate::environment::map_size::MapSize;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

pub struct Coordinate {
    /// 行坐标
    y: usize,
    /// 列坐标
    x: usize,
}

impl Coordinate {
    /// 创建一个新的坐标
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    /// 计算本坐标在指定方向上的新坐标
    fn get_coordinate_shifted_by_relation<R>(
        &self,
        height_width: (usize, usize),
        relation: R,
    ) -> Self
    where
        R: HexaRelational,
    {
        // 获取方向的坐标偏移
        let coordinate_shift = R::from_relation_to_coordinate_shift()[&relation];

        // 计算新的 y 和 x 坐标，考虑宽高的环绕（模数）效果
        let new_y =
            (self.y as isize + coordinate_shift.dy()).rem_euclid(height_width.0 as isize) as usize;
        let new_x =
            (self.x as isize + coordinate_shift.dx()).rem_euclid(height_width.1 as isize) as usize;

        // 返回新的坐标
        Self::new(new_y, new_x)
    }

    /// 获取几何关系对应的坐标
    ///
    /// 该方法会根据传入的关系类型 `R` 返回一个包含各个方向对应坐标的映射表。
    /// 目前，`R` 可能表示邻居关系或对角关系。
    pub fn get_relations_map<R>(&self, height_width: (usize, usize)) -> HashMap<R, Coordinate>
    where
        R: HexaRelational,
    {
        // 从关系类型 `R` 获取方向到坐标偏移的映射表
        R::from_relation_to_coordinate_shift()
            .iter()
            .map(|(&relation, _)| {
                // 对每个关系 (例如邻居或对角)，计算该方向上的新坐标
                let relation_coordinate =
                    Self::get_coordinate_shifted_by_relation(self, height_width, relation);
                // 将关系和新坐标作为键值对加入到 HashMap 中
                (relation, relation_coordinate)
            })
            .collect::<HashMap<R, Coordinate>>() // 收集所有键值对，生成最终的 HashMap
    }

    /// 缩放坐标
    pub fn scale(&self, scale: usize) -> Self {
        Self::new(self.y * scale, self.x * scale)
    }

    /// 带环绕效果的加法运算
    fn add_wrapping(self, other: Self, map_size: &MapSize) -> Self {
        Self {
            y: (self.y + other.y) % map_size.height(),
            x: (self.x + other.x) % map_size.width(),
        }
    }
    /// 带环绕效果的减法运算
    fn sub_wrapping(self, other: Self, map_size: &MapSize) -> Self {
        Self {
            y: ((self.y as isize - other.y as isize).rem_euclid(map_size.height() as isize))
                as usize,
            x: ((self.x as isize - other.x as isize).rem_euclid(map_size.width() as isize))
                as usize,
        }
    }

    pub fn x(&self) -> &usize {
        &self.x
    }

    pub fn y(&self) -> &usize {
        &self.y
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

impl Add<(Self, &MapSize)> for Coordinate {
    type Output = Self;

    fn add(self, (other, map_size): (Self, &MapSize)) -> Self {
        self.add_wrapping(other, map_size)
    }
}

impl Add<(CoordinateShift, &MapSize)> for Coordinate {
    type Output = Self;

    fn add(self, (shift, map_size): (CoordinateShift, &MapSize)) -> Self {
        Self {
            y: ((self.y as isize + shift.dy()).rem_euclid(map_size.height() as isize)) as usize,
            x: ((self.x as isize + shift.dx()).rem_euclid(map_size.width() as isize)) as usize,
        }
    }
}

impl Sub<(CoordinateShift, &MapSize)> for Coordinate {
    type Output = Self;

    fn sub(self, (shift, map_size): (CoordinateShift, &MapSize)) -> Self {
        Self {
            y: ((self.y as isize - shift.dy()).rem_euclid(map_size.height() as isize)) as usize,
            x: ((self.x as isize - shift.dx()).rem_euclid(map_size.width() as isize)) as usize,
        }
    }
}

impl Sub<(Self, &MapSize)> for Coordinate {
    type Output = Self;

    fn sub(self, (other, map_size): (Self, &MapSize)) -> Self {
        self.sub_wrapping(other, map_size)
    }
}

impl Mul<usize> for Coordinate {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            y: self.y * scalar,
            x: self.x * scalar,
        }
    }
}

impl Mul for Coordinate {
    type Output = usize;

    fn mul(self, other: Self) -> usize {
        self.y * other.y + self.x * other.x
    }
}
