use crate::environment::hexagon::diagonal_relation::DiagonalRelation;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HexSpoke<T> {
    center: T,
    neighbors: HashMap<DiagonalRelation, T>,
}

impl<T> HexSpoke<T> {
    pub fn new(center: T, neighbors: HashMap<DiagonalRelation, T>) -> Self {
        Self { center, neighbors }
    }

    pub fn center(&self) -> &T {
        &self.center
    }

    pub fn neighbors(&self) -> &HashMap<DiagonalRelation, T> {
        &self.neighbors
    }

    pub fn into_parts(self) -> (T, HashMap<DiagonalRelation, T>) {
        (self.center, self.neighbors)
    }
}
