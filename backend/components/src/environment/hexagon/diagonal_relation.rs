use crate::environment::coordinate_shift::CoordinateShift;
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
            (DiagonalRelation::Degree30, CoordinateShift::new(-2, 1)),
            (DiagonalRelation::Degree90, CoordinateShift::new(-1, 2)),
            (DiagonalRelation::Degree150, CoordinateShift::new(1, 1)),
            (DiagonalRelation::Degree210, CoordinateShift::new(2, -1)),
            (DiagonalRelation::Degree270, CoordinateShift::new(1, -2)),
            (DiagonalRelation::Degree330, CoordinateShift::new(-1, -1)),
        ])
    }
}
