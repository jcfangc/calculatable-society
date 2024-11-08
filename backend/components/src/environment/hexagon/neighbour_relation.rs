use crate::environment::coordinate_offset::CoordinateShift;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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
            (
                NeighbourRelation::Degree0,
                CoordinateShift { dy: -1, dx: 0 },
            ),
            (
                NeighbourRelation::Degree60,
                CoordinateShift { dy: -1, dx: 1 },
            ),
            (
                NeighbourRelation::Degree120,
                CoordinateShift { dy: 0, dx: 1 },
            ),
            (
                NeighbourRelation::Degree180,
                CoordinateShift { dy: 1, dx: 0 },
            ),
            (
                NeighbourRelation::Degree240,
                CoordinateShift { dy: 1, dx: -1 },
            ),
            (
                NeighbourRelation::Degree300,
                CoordinateShift { dy: 0, dx: -1 },
            ),
        ])
    }
}
