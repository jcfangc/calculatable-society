use crate::environment::direction_offset::DirectionOffset;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Hexagon {}

pub trait Direction: Hexagon + Copy + Eq + Hash {
    fn offsets() -> HashMap<Self, DirectionOffset>
    where
        Self: Sized;
}
