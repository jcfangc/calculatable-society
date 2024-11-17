use serde::Serialize;
use std::f64::consts::PI;
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

type Radians = f64;
type Degrees = f64;

/// 方向对应的坐标偏移量
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct CoordinateShift {
    dy: isize,
    dx: isize,
}

impl CoordinateShift {
    /// 创建一个新的坐标偏移量
    pub fn new(dy: isize, dx: isize) -> Self {
        Self { dy, dx }
    }

    /// 获取坐标偏移量的相反方向
    pub fn reverse(&self) -> Self {
        Self::new(-self.dy, -self.dx)
    }

    /// 获取 dy 坐标偏移量
    pub fn dy(&self) -> isize {
        self.dy
    }

    /// 获取 dx 坐标偏移量
    pub fn dx(&self) -> isize {
        self.dx
    }

    /// 计算两个坐标偏移量之间的角度差（以弧度为单位）
    ///
    /// ### 参数
    /// - `self`：第一个向量（当前实例）。
    /// - `other`：第二个向量。
    ///
    /// ### 返回值
    /// 返回从 `self` 到 `other` 的有符号角度差（以弧度为单位），范围为 [-π, π]。
    ///
    /// ### 符号意义
    /// - 返回值为正数时，表示从 `self` 到 `other` 为逆时针旋转（正方向）。
    /// - 返回值为负数时，表示从 `self` 到 `other` 为顺时针旋转（负方向）。
    /// - 返回值为 0 时，表示两个向量方向相同或重合。
    pub fn angle_between(self, other: Self) -> Radians {
        let dot = (self.dx * other.dx + self.dy * other.dy) as f64;
        let cross = (self.dx * other.dy - self.dy * other.dx) as f64;
        let angle = cross.atan2(dot);
        angle
    }

    /// 将弧度转换为角度
    pub fn to_degrees(radians: Radians) -> Degrees {
        radians * (180.0 / PI)
    }

    /// 将角度转换为弧度
    pub fn to_radians(degrees: Degrees) -> Radians {
        degrees * (PI / 180.0)
    }

    /// 计算当前坐标偏移量的模长
    ///
    /// ### 适用于等边六边形地图
    /// - 每个单元格表示一个六边形单元。
    /// - 返回模长，即从起点到目标点所需的最小步数。
    ///
    /// ### 返回值
    /// 返回值为 `usize` 类型，表示非负整数的步数。
    pub fn magnitude(&self) -> usize {
        let dx = self.dx.abs() as usize;
        let dy = self.dy.abs() as usize;
        let dz = (-(self.dx + self.dy)).abs() as usize; // 第三个方向，用于六边形坐标系统
        *[dx, dy, dz].iter().max().unwrap()
    }
}

// 实现加法运算
impl Add for CoordinateShift {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.dy + other.dy, self.dx + other.dx)
    }
}

// 实现减法运算
impl Sub for CoordinateShift {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.dy - other.dy, self.dx - other.dx)
    }
}

// 实现标量相乘
impl Mul<isize> for CoordinateShift {
    type Output = Self;

    fn mul(self, scalar: isize) -> Self::Output {
        Self::new(self.dy * scalar, self.dx * scalar)
    }
}

// 实现浮点数标量相乘：CoordinateShift * f64
impl Mul<f64> for CoordinateShift {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(
            (self.dy as f64 * scalar).round() as isize,
            (self.dx as f64 * scalar).round() as isize,
        )
    }
}

// 实现浮点数标量相乘：f64 * CoordinateShift
impl Mul<CoordinateShift> for f64 {
    type Output = CoordinateShift;

    fn mul(self, shift: CoordinateShift) -> CoordinateShift {
        CoordinateShift::new(
            (shift.dy as f64 * self).round() as isize,
            (shift.dx as f64 * self).round() as isize,
        )
    }
}

// 实现 Sum trait 以支持 .sum() 操作
impl Sum for CoordinateShift {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new(0, 0), |acc, shift| acc + shift)
    }
}
