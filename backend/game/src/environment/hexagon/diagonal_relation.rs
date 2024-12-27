use crate::environment::hexagon::hex_displacemant::HexDisplacement;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) enum DiagonalRelation {
    Degree30 = 30,
    Degree90 = 90,
    Degree150 = 150,
    Degree210 = 210,
    Degree270 = 270,
    Degree330 = 330,
}

impl HexaRelational for DiagonalRelation {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, HexDisplacement> {
        HashMap::from([
            (DiagonalRelation::Degree30, HexDisplacement::new(-2, 1)),
            (DiagonalRelation::Degree90, HexDisplacement::new(-1, 2)),
            (DiagonalRelation::Degree150, HexDisplacement::new(1, 1)),
            (DiagonalRelation::Degree210, HexDisplacement::new(2, -1)),
            (DiagonalRelation::Degree270, HexDisplacement::new(1, -2)),
            (DiagonalRelation::Degree330, HexDisplacement::new(-1, -1)),
        ])
    }

    fn opposite_pairs() -> [(Self, Self); 3] {
        [
            (DiagonalRelation::Degree30, DiagonalRelation::Degree210),
            (DiagonalRelation::Degree90, DiagonalRelation::Degree270),
            (DiagonalRelation::Degree150, DiagonalRelation::Degree330),
        ]
    }
}
