use crate::environment::coordinate_shift::CoordinateShift;
use std::collections::HashMap;
use std::hash::Hash;

pub(crate) trait HexaRelational: Copy + Eq + Hash {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, CoordinateShift>;
}
