use crate::environment::hexagon::neighbour_relation::NeighbourRelation;
use std::collections::HashMap;

/// 作为一个盛放 中心单元 + 邻居单元 相关数据<T>的容器
#[derive(Clone, Debug)]
pub(crate) struct HexBlock<T> {
    center: T,
    neighbors: HashMap<NeighbourRelation, T>,
}

impl<T> HexBlock<T> {
    pub(crate) fn new(center: T, neighbors: HashMap<NeighbourRelation, T>) -> Self {
        Self { center, neighbors }
    }

    pub(crate) fn center(&self) -> &T {
        &self.center
    }

    pub(crate) fn neighbors(&self) -> &HashMap<NeighbourRelation, T> {
        &self.neighbors
    }

    pub(crate) fn into_parts(self) -> (T, HashMap<NeighbourRelation, T>) {
        (self.center, self.neighbors)
    }

    /// 从邻居获取指定方向的信息
    ///
    /// ### 参数
    /// - `relation`: 指定的邻居方向。
    ///
    /// ### 返回值
    /// 返回邻居方向上的数据，如果不存在则 panic。
    pub(crate) fn get_from_neighbours(&self, relation: NeighbourRelation) -> T
    where
        T: Clone,
    {
        self.neighbors
            .get(&relation)
            .cloned() // 从引用克隆为值类型
            .unwrap_or_else(|| panic!("{:?} 角度邻居不存在于信息块中", relation))
    }
}
