use std::collections::HashMap;

use crate::environment::coordinate_shift::CoordinateShift;
use serde::Serialize;

use super::neighbour_relation::NeighbourRelation;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize)]
pub struct HexUnit {
    /// 本单元所含物质的摩尔数
    mole: usize,
    /// 本单元当下保持的运动状态
    movement: CoordinateShift,
}

impl HexUnit {
    pub fn new(mole: usize, movement: CoordinateShift) -> Self {
        Self {
            mole: mole,
            movement: movement,
        }
    }

    pub fn mole(&self) -> usize {
        self.mole
    }

    pub fn movement(&self) -> CoordinateShift {
        self.movement
    }

    pub fn set_mole(&mut self, mole: usize) {
        self.mole = mole;
    }

    pub fn set_movement(&mut self, movement: CoordinateShift) {
        self.movement = movement;
    }

    pub fn adjust_mole(&mut self, mole: isize) {
        self.mole = (self.mole as isize + mole as isize) as usize;
    }

    pub fn adjust_movement(&mut self, movement: CoordinateShift) {
        self.movement = self.movement + movement;
    }
}

impl Default for HexUnit {
    fn default() -> Self {
        Self {
            mole: 0,
            movement: CoordinateShift::new(0, 0),
        }
    }
}
