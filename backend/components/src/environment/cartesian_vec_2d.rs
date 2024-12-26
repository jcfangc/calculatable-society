use serde::Serialize;
use std::f64::EPSILON;
use std::hash::{Hash, Hasher};
use std::ops::Add;

type Radians = f64;

#[derive(Clone, Copy, Debug, Serialize, Default)]
pub(crate) struct CartesianVec2D {
    /// 行坐标
    y: f64,
    /// 列坐标
    x: f64,
}

impl CartesianVec2D {
    /// 创建一个新的坐标
    pub(crate) fn new(y: f64, x: f64) -> Self {
        Self { y, x }
    }

    /// 缩放坐标
    pub(crate) fn scale(&self, scale: f64) -> Self {
        Self::new(self.y * scale, self.x * scale)
    }

    /// 获取行坐标
    pub(crate) fn y(&self) -> f64 {
        self.y
    }

    /// 获取列坐标
    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    /// 计算向量的模长（长度）
    ///
    /// ### 返回值
    /// 返回当前向量的欧几里得模长。
    pub(crate) fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// 计算向量的模长平方（避免开方操作）
    ///
    /// ### 返回值
    /// 返回当前向量模长的平方值，适合需要比较大小但不需要具体长度的场景。
    pub(crate) fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// 计算两个坐标偏移量之间的角度差（以弧度为单位）
    ///
    /// ### 参数
    /// - self：第一个向量（当前实例）。
    /// - other：第二个向量。
    ///
    /// ### 返回值
    /// 返回从 self 到 other 的有符号角度差（以弧度为单位），范围为 [-π, π]。
    ///
    /// ### 符号意义
    /// - 返回值为正数时，表示从 self 到 other 为逆时针旋转（正方向）。
    /// - 返回值为负数时，表示从 self 到 other 为顺时针旋转（负方向）。
    /// - 返回值为 0 时，表示两个向量方向相同或重合。
    pub(crate) fn angle_between(self, other: Self) -> Radians {
        let dot = self.x() * other.x() + self.y() * other.y();
        let cross = self.x() * other.y() - self.y() * other.x();
        cross.atan2(dot)
    }
}

impl Add for CartesianVec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.y + other.y, self.x + other.x)
    }
}

// 实现 Eq
impl Eq for CartesianVec2D {}

// 实现 PartialEq，使用 EPSILON 判断浮点数相等
impl PartialEq for CartesianVec2D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

// 实现 Hash，确保浮点数小范围误差内的数值具有相同的哈希值
impl Hash for CartesianVec2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 将浮点数四舍五入到 EPSILON 精度后进行哈希计算
        let x_rounded = (self.x / EPSILON).round() as i64;
        let y_rounded = (self.y / EPSILON).round() as i64;
        x_rounded.hash(state);
        y_rounded.hash(state);
    }
}
