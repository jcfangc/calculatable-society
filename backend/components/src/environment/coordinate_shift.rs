use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

/// 方向对应的坐标偏移量
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
