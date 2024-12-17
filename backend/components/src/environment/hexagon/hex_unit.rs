use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::diffuse_info::DiffuseInfo;
use crate::environment::hexagon::hex_block::HexBlock;
use crate::environment::hexagon::unit_change::UnitChange;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize)]
pub(crate) struct HexUnit {
    /// 本单元所含物质的摩尔数
    mole: usize,
    /// 本单元当下保持的运动状态
    movement: CoordinateShift,
}

impl HexUnit {
    pub(crate) fn new(mole: usize, movement: CoordinateShift) -> Self {
        Self { mole, movement }
    }

    pub(crate) fn mole(&self) -> usize {
        self.mole
    }

    pub(crate) fn movement(&self) -> CoordinateShift {
        self.movement
    }

    pub(crate) fn set_mole(&mut self, mole: usize) {
        self.mole = mole;
    }

    pub(crate) fn set_movement(&mut self, movement: CoordinateShift) {
        self.movement = movement;
    }

    pub(crate) fn adjust_mole(&mut self, mole: isize) {
        self.mole = (self.mole as isize + mole as isize) as usize;
    }

    pub(crate) fn adjust_movement(&mut self, movement: CoordinateShift) {
        self.movement = self.movement + movement;
    }

    pub(crate) fn fit_change(&mut self, unit_change: UnitChange) {
        self.mole = (self.mole as isize + unit_change.mole_change() as isize) as usize;
        self.movement = self.movement + unit_change.movement_change();
    }

    pub(crate) fn diffuse(&self, block_info: &HexBlock<DiffuseInfo>) -> HexBlock<UnitChange> {}
}

impl Default for HexUnit {
    fn default() -> Self {
        Self {
            mole: 0,
            movement: CoordinateShift::new(0, 0),
        }
    }
}
