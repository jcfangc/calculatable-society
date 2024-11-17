use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum NeighbourRelation {
    Degree0,
    Degree60,
    Degree120,
    Degree180,
    Degree240,
    Degree300,
}

impl HexaRelational for NeighbourRelation {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, CoordinateShift> {
        HashMap::from([
            (NeighbourRelation::Degree0, CoordinateShift::new(-1, 0)),
            (NeighbourRelation::Degree60, CoordinateShift::new(-1, 1)),
            (NeighbourRelation::Degree120, CoordinateShift::new(0, 1)),
            (NeighbourRelation::Degree180, CoordinateShift::new(1, 0)),
            (NeighbourRelation::Degree240, CoordinateShift::new(1, -1)),
            (NeighbourRelation::Degree300, CoordinateShift::new(0, -1)),
        ])
    }
}
