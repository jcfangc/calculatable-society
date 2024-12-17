use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HexBlock<T> {
    center: T,
    neighbors: HashMap<NeighbourRelation, T>,
}

impl<T> HexBlock<T> {
    pub fn new(center: T, neighbors: HashMap<NeighbourRelation, T>) -> Self {
        Self { center, neighbors }
    }

    pub fn center(&self) -> &T {
        &self.center
    }

    pub fn neighbors(&self) -> &HashMap<NeighbourRelation, T> {
        &self.neighbors
    }

    pub fn into_parts(self) -> (T, HashMap<NeighbourRelation, T>) {
        (self.center, self.neighbors)
    }
}
