use crate::environment::cartesian_vec_2d::CartesianVec2D;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize)]
pub(crate) struct UnitChange {
    /// 本单元所含物质的摩尔数
    mole_change: isize,
    /// 本单元当下保持的运动状态
    movement_change: CartesianVec2D,
}

impl UnitChange {
    /// 创建一个新的单元变化
    pub(crate) fn new(mole_change: isize, movement_change: CartesianVec2D) -> Self {
        Self {
            mole_change,
            movement_change,
        }
    }

    /// 获取摩尔数变化
    pub(crate) fn mole_change(&self) -> isize {
        self.mole_change
    }

    /// 获取运动状态变化
    pub(crate) fn movement_change(&self) -> CartesianVec2D {
        self.movement_change
    }

    /// 设置摩尔数变化
    pub(crate) fn set_mole_change(&mut self, mole_change: isize) {
        self.mole_change = mole_change;
    }

    /// 设置运动状态变化
    pub(crate) fn set_movement_change(&mut self, movement_change: CartesianVec2D) {
        self.movement_change = movement_change;
    }

    /// 设置单元变化
    pub(crate) fn set_change(&mut self, unit_change: UnitChange) {
        self.mole_change = unit_change.mole_change;
        self.movement_change = unit_change.movement_change;
    }

    /// 累积摩尔数变化
    pub(crate) fn accumulate_mole_change(&mut self, mole_change: isize) {
        self.mole_change = self.mole_change + mole_change;
    }

    /// 累积运动状态变化
    pub(crate) fn accumulate_movement_change(&mut self, movement_change: CartesianVec2D) {
        self.movement_change = self.movement_change + movement_change;
    }

    /// 累积单元变化
    pub(crate) fn accumulate_change(&mut self, other: &UnitChange) {
        self.mole_change = self.mole_change + other.mole_change;
        self.movement_change = self.movement_change + other.movement_change;
    }
}

impl Default for UnitChange {
    fn default() -> Self {
        Self {
            mole_change: 0,
            movement_change: CartesianVec2D::new(0.0, 0.0),
        }
    }
}
