use crate::environment::hexagon::hex_displacemant::HexDisplacement;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) trait HexaRelational: Copy + Eq + Hash + Debug {
    /// 从关系类型到坐标偏移量的映射，
    /// 偏移是指从中心单元到特定邻居单元再六边形网格坐标上的偏移
    fn from_relation_to_coordinate_shift() -> HashMap<Self, HexDisplacement>;

    /// 根据给定的 NeighbourRelation 返回对应的 HexCoordShift
    fn to_coordinate_shift(relation: Self) -> HexDisplacement {
        Self::from_relation_to_coordinate_shift()
            .get(&relation)
            .cloned() // 克隆值以便返回
            .unwrap_or_else(|| panic!("不存在的 NeighbourRelation: {:?}", relation))
    }

    /// 返回相对的，相隔180度的关系对，
    /// 这些关系对的角度关系特殊，相互之间的数值易用于标量计算
    fn opposite_pairs() -> [(Self, Self); 3];
}
