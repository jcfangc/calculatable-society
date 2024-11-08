use crate::environment::coordinate_offset::CoordinateShift;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum DiagonalRelation {
    Degree30,
    Degree90,
    Degree150,
    Degree210,
    Degree270,
    Degree330,
}

impl HexaRelational for DiagonalRelation {
    fn from_relation_to_coordinate_shift() -> HashMap<Self, CoordinateShift> {
        HashMap::from([
            (
                DiagonalRelation::Degree30,
                CoordinateShift { dy: -2, dx: 1 },
            ),
            (
                DiagonalRelation::Degree90,
                CoordinateShift { dy: -1, dx: 2 },
            ),
            (
                DiagonalRelation::Degree150,
                CoordinateShift { dy: 1, dx: 1 },
            ),
            (
                DiagonalRelation::Degree210,
                CoordinateShift { dy: 2, dx: -1 },
            ),
            (
                DiagonalRelation::Degree270,
                CoordinateShift { dy: 1, dx: -2 },
            ),
            (
                DiagonalRelation::Degree330,
                CoordinateShift { dy: -1, dx: -1 },
            ),
        ])
    }
}
