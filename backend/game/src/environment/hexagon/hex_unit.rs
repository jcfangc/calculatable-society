use crate::environment::cartesian_vec_2d::CartesianVec2D;
use crate::environment::diffuse_info::DiffuseInfo;
use crate::environment::hexagon::hex_block::HexBlock;
use crate::environment::hexagon::hex_displacemant::HexDisplacement;
use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use crate::environment::hexagon::unit_change::UnitChange;
use serde::Serialize;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize)]
pub(crate) struct HexUnit {
    /// 本单元所含物质的摩尔数
    mole: usize,
    /// 本单元当下的运动状态（实际上是未消耗的完的势能，但是带有方向）
    movement: CartesianVec2D,
}

/// 关于字段的基本操作
impl HexUnit {
    pub(crate) fn new(mole: usize, movement: CartesianVec2D) -> Self {
        Self { mole, movement }
    }

    pub(crate) fn mole(&self) -> usize {
        self.mole
    }

    pub(crate) fn movement(&self) -> CartesianVec2D {
        self.movement
    }

    pub(crate) fn set_mole(&mut self, mole: usize) {
        self.mole = mole;
    }

    pub(crate) fn set_movement(&mut self, movement: CartesianVec2D) {
        self.movement = movement;
    }

    pub(crate) fn adjust_mole(&mut self, mole: isize) {
        self.mole = (self.mole as isize + mole as isize) as usize;
    }

    pub(crate) fn adjust_movement(&mut self, movement: CartesianVec2D) {
        self.movement = self.movement + movement;
    }

    pub(crate) fn fit_change(&mut self, unit_change: UnitChange) {
        self.mole = (self.mole as isize + unit_change.mole_change() as isize) as usize;
        self.movement = self.movement + unit_change.movement_change();
    }
}

/// 关于扩散逻辑的集合
impl HexUnit {
    pub(crate) fn diffuse(
        &self,
        fluidity: f64,
        block_info: &HexBlock<DiffuseInfo>,
    ) -> HexBlock<UnitChange> {
        // 1. 计算三对邻居势能差
        let reduced_potential = self.calculate_reduced_potential(block_info);

        // 2. 累加自身movement到笛卡尔坐标
        let total_cartesian_shift = self.calculate_total_cartesian_shift(&reduced_potential);

        // 3. 为每个邻居计算UnitChange
        let neighbour_changes =
            self.calculate_neighbour_changes(fluidity, block_info, total_cartesian_shift);

        // 4. 累加到 self_change 并做守恒性修正
        let self_change = self.calculate_self_change(&neighbour_changes);

        // 5. 组合成 HexBlock
        HexBlock::new(self_change, neighbour_changes)
    }

    fn calculate_reduced_potential(
        &self,
        block_info: &HexBlock<DiffuseInfo>,
    ) -> [(HexDisplacement, f64); 3] {
        NeighbourRelation::opposite_pairs().map(|(dir_a, dir_b)| {
            let potential_a = block_info.get_from_neighbours(dir_a).potential();
            let potential_b = block_info.get_from_neighbours(dir_b).potential();

            let delta = potential_a - potential_b;

            // 根据势能差计算反向位移
            let shift =
                NeighbourRelation::to_coordinate_shift(if delta > 0.0 { dir_a } else { dir_b })
                    .reverse();

            (shift, delta.abs())
        })
    }

    fn calculate_total_cartesian_shift(
        &self,
        reduced_potential: &[(HexDisplacement, f64); 3],
    ) -> CartesianVec2D {
        self.movement() // 本单元当前的留存动态
            + reduced_potential
                .iter()
                .map(|(hex_shift, potential)| hex_shift.to_cartesian().scale(*potential))
                .reduce(|acc, vec| acc + vec) // 累加所有方向
                .unwrap_or_else(|| panic!("无法将指定HexCoordShift投影到笛卡尔空间合并！"))
    }

    fn calculate_neighbour_changes(
        &self,
        fluidity: f64,
        block_info: &HexBlock<DiffuseInfo>,
        total_cartesian_shift: CartesianVec2D,
    ) -> HashMap<NeighbourRelation, UnitChange> {
        NeighbourRelation::from_relation_to_coordinate_shift()
            .iter()
            .map(|(&relation, shift)| {
                let weight = (2.0 * PI
                    - shift
                        .to_cartesian()
                        .angle_between(total_cartesian_shift)
                        .abs())
                    / (9.0 * PI);

                let mut partial_mole = (self.mole() as f64 * weight * fluidity).round() as usize;
                let mut partial_movement = shift
                    .to_cartesian()
                    .scale(total_cartesian_shift.magnitude() * weight);

                let barrier = block_info.get_from_neighbours(relation).potential();

                if partial_movement.magnitude() > barrier {
                    partial_movement = shift
                        .to_cartesian()
                        .scale(partial_movement.magnitude() - barrier);
                } else {
                    partial_movement = shift.to_cartesian().scale(0.0);
                    partial_mole = 0;
                }

                let change = UnitChange::new(partial_mole as isize, partial_movement);
                (relation, change)
            })
            .collect()
    }

    fn calculate_self_change(
        &self,
        neighbour_changes: &HashMap<NeighbourRelation, UnitChange>,
    ) -> UnitChange {
        let mut self_change = UnitChange::new(0, CartesianVec2D::new(0.0, 0.0));

        for change in neighbour_changes.values() {
            self_change.accumulate_change(change);
        }

        // 保持守恒性：反转符号
        self_change.set_mole_change(-self_change.mole_change());
        self_change.set_movement_change(self_change.movement_change().scale(-1.0));

        self_change
    }
}

impl Default for HexUnit {
    fn default() -> Self {
        Self {
            mole: 0,
            movement: CartesianVec2D::new(0.0, 0.0),
        }
    }
}
