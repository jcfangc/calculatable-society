use crate::environment::cartesian_vec_2d::CartesianVec2D;
use crate::game_context::GameContext;
use serde::Serialize;
use std::iter::Sum;
use std::ops::{Add, Mul, Sub};

type Radians = f64;
type Degrees = f64;

/// 方向对应的坐标偏移量
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Default)]
pub(crate) struct HexDisplacement {
    dy: isize,
    dx: isize,
}

impl HexDisplacement {
    /// 创建一个新的坐标偏移量
    pub(crate) fn new(dy: isize, dx: isize) -> Self {
        Self { dy, dx }
    }

    /// 获取坐标偏移量的相反方向
    pub(crate) fn reverse(&self) -> Self {
        Self::new(-self.dy, -self.dx)
    }

    /// 获取 dy 坐标偏移量
    pub(crate) fn dy(&self) -> isize {
        self.dy
    }

    /// 获取 dx 坐标偏移量
    pub(crate) fn dx(&self) -> isize {
        self.dx
    }

    /// 将当前偏移量变换到笛卡尔空间
    ///
    /// ### 返回值
    /// 返回对应的笛卡尔坐标系中的向量。
    pub(crate) fn to_cartesian(&self) -> CartesianVec2D {
        // 获取基础向量并进行线性组合
        let x_component = GameContext::get_x_base_vector().scale(self.dx as f64);
        let y_component = GameContext::get_y_base_vector().scale(self.dy as f64);

        x_component + y_component
    }

    /// 计算当前坐标偏移量的模长
    ///
    /// ### 适用于等边六边形地图
    /// - 每个单元格表示一个六边形单元。
    /// - 返回模长，即从起点到目标点所需的最小步数。
    ///
    /// ### 返回值
    /// 返回值为 `usize` 类型，表示非负整数的步数。
    pub(crate) fn magnitude(&self) -> usize {
        let dx = self.dx.abs() as usize;
        let dy = self.dy.abs() as usize;
        let dz = (-(self.dx + self.dy)).abs() as usize; // 第三个方向，用于六边形坐标系统
        *[dx, dy, dz].iter().max().unwrap()
    }
}

// 实现加法运算
impl Add for HexDisplacement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.dy + other.dy, self.dx + other.dx)
    }
}

// 实现减法运算
impl Sub for HexDisplacement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.dy - other.dy, self.dx - other.dx)
    }
}

// 实现标量相乘
impl Mul<isize> for HexDisplacement {
    type Output = Self;

    fn mul(self, scalar: isize) -> Self::Output {
        Self::new(self.dy * scalar, self.dx * scalar)
    }
}

// 实现浮点数标量相乘：CoordinateShift * f64
impl Mul<f64> for HexDisplacement {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(
            (self.dy as f64 * scalar).round() as isize,
            (self.dx as f64 * scalar).round() as isize,
        )
    }
}

// 实现浮点数标量相乘：f64 * CoordinateShift
impl Mul<HexDisplacement> for f64 {
    type Output = HexDisplacement;

    fn mul(self, shift: HexDisplacement) -> HexDisplacement {
        HexDisplacement::new(
            (shift.dy as f64 * self).round() as isize,
            (shift.dx as f64 * self).round() as isize,
        )
    }
}

// 实现 Sum trait 以支持 .sum() 操作
impl Sum for HexDisplacement {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new(0, 0), |acc, shift| acc + shift)
    }
}
