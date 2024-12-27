use crate::environment::hexagon::hex_displacemant::HexDisplacement;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) enum NeighbourRelation {
    Degree0 = 0,
    Degree60 = 60,
    Degree120 = 120,
    Degree180 = 180,
    Degree240 = 240,
    Degree300 = 300,
}

impl HexaRelational for NeighbourRelation {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, HexDisplacement> {
        HashMap::from([
            (NeighbourRelation::Degree0, HexDisplacement::new(-1, 0)),
            (NeighbourRelation::Degree60, HexDisplacement::new(-1, 1)),
            (NeighbourRelation::Degree120, HexDisplacement::new(0, 1)),
            (NeighbourRelation::Degree180, HexDisplacement::new(1, 0)),
            (NeighbourRelation::Degree240, HexDisplacement::new(1, -1)),
            (NeighbourRelation::Degree300, HexDisplacement::new(0, -1)),
        ])
    }

    fn opposite_pairs() -> [(Self, Self); 3] {
        [
            (NeighbourRelation::Degree0, NeighbourRelation::Degree180),
            (NeighbourRelation::Degree60, NeighbourRelation::Degree240),
            (NeighbourRelation::Degree120, NeighbourRelation::Degree300),
        ]
    }
}
