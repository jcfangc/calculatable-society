use crate::environment::coordinate_shift::CoordinateShift;
use crate::environment::diffuse_info::DiffuseInfo;
use crate::environment::hexagon::hex_block::HexBlock;
use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use crate::environment::hexagon::unit_change::UnitChange;
use crate::shared::subtance_type::SubstanceType;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize)]
pub(crate) struct HexUnit {
    /// 本单元所含物质的摩尔数
    mole: usize,
    /// 本单元当下保持的运动状态
    movement: CoordinateShift,
}

impl HexUnit {
    pub(crate) fn new(mole: usize, movement: CoordinateShift) -> Self {
        Self { mole, movement }
    }

    pub(crate) fn mole(&self) -> usize {
        self.mole
    }

    pub(crate) fn movement(&self) -> CoordinateShift {
        self.movement
    }

    pub(crate) fn set_mole(&mut self, mole: usize) {
        self.mole = mole;
    }

    pub(crate) fn set_movement(&mut self, movement: CoordinateShift) {
        self.movement = movement;
    }

    pub(crate) fn adjust_mole(&mut self, mole: isize) {
        self.mole = (self.mole as isize + mole as isize) as usize;
    }

    pub(crate) fn adjust_movement(&mut self, movement: CoordinateShift) {
        self.movement = self.movement + movement;
    }

    pub(crate) fn fit_change(&mut self, unit_change: UnitChange) {
        self.mole = (self.mole as isize + unit_change.mole_change() as isize) as usize;
        self.movement = self.movement + unit_change.movement_change();
    }

    pub(crate) fn diffuse(
        &self,
        subtance_type: SubstanceType,
        block_info: &HexBlock<DiffuseInfo>,
    ) -> HexBlock<UnitChange> {
        let directions = [
            (NeighbourRelation::Degree0, NeighbourRelation::Degree180),
            (NeighbourRelation::Degree60, NeighbourRelation::Degree240),
            (NeighbourRelation::Degree120, NeighbourRelation::Degree300),
        ];

        let reduced_potential: [(NeighbourRelation, f64); 3] = directions.map(|(dir_a, dir_b)| {
            let potential_a = block_info
                .neighbors()
                .get(&dir_a)
                .unwrap_or_else(|| panic!("{:?} 角度邻居不存在于信息块中", dir_a))
                .potential();

            let potential_b = block_info
                .neighbors()
                .get(&dir_b)
                .unwrap_or_else(|| panic!("{:?} 角度邻居不存在于信息块中", dir_b))
                .potential();

            let delta = potential_a - potential_b;
            if delta > 0.0 {
                (dir_a, delta)
            } else {
                (dir_b, -delta)
            }
        });
    }
}

impl Default for HexUnit {
    fn default() -> Self {
        Self {
            mole: 0,
            movement: CoordinateShift::new(0, 0),
        }
    }
}
