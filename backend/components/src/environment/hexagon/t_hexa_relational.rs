use crate::environment::coordinate_offset::CoordinateShift;
use std::collections::HashMap;
use std::hash::Hash;

pub trait HexaRelational: Copy + Eq + Hash {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, CoordinateShift>;
}
