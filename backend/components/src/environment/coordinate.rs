use crate::environment::hexagon::t_hexa_distanced::HexaDistanced;
use crate::environment::hexagon::t_hexa_relational::HexaRelational;
use crate::environment::layer::Layer;
use std::collections::HashMap;

pub struct Coordinate {
    /// 行坐标
    pub y: usize,
    /// 列坐标
    pub x: usize,
}

impl Coordinate {
    /// 创建一个新的坐标
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    /// 计算本坐标在指定方向上的新坐标
    fn get_coordinate_shifted_by_relation<R>(&self, layer: &Layer, relation: R) -> Self
    where
        R: HexaRelational,
    {
        // 获取方向的坐标偏移
        let coordinate_shift = R::from_relation_to_coordinate_shift()[&relation];

        // 计算新的 y 和 x 坐标，考虑宽高的环绕（模数）效果
        let new_y =
            (self.y as isize + coordinate_shift.dy).rem_euclid(layer.height as isize) as usize;
        let new_x =
            (self.x as isize + coordinate_shift.dx).rem_euclid(layer.width as isize) as usize;

        // 返回新的坐标
        Self::new(new_y, new_x)
    }

    /// 获取几何关系对应的坐标
    ///
    /// 该方法会根据传入的关系类型 `R` 返回一个包含各个方向对应坐标的映射表。
    /// 目前，`R` 可能表示邻居关系或对角关系。
    pub fn get_relations_map<R>(&self, layer: &Layer) -> HashMap<R, Coordinate>
    where
        R: HexaRelational,
    {
        // 从关系类型 `R` 获取方向到坐标偏移的映射表
        R::from_relation_to_coordinate_shift()
            .iter()
            .map(|(&relation, _)| {
                // 对每个关系 (例如邻居或对角)，计算该方向上的新坐标
                let relation_coordinate =
                    Self::get_coordinate_shifted_by_relation(self, layer, relation);
                // 将关系和新坐标作为键值对加入到 HashMap 中
                (relation, relation_coordinate)
            })
            .collect::<HashMap<R, Coordinate>>() // 收集所有键值对，生成最终的 HashMap
    }
}

impl HexaDistanced for Coordinate {
    /// 立方体坐标下的隐藏第三轴
    fn z(&self) -> isize {
        -(self.x as isize + self.y as isize)
    }

    /// 计算两坐标之间的六边形距离
    fn distance(&self, other: &Coordinate) -> usize {
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        let dz = (self.z() - other.z()).abs();
        ((dx + dy + dz) / 2) as usize
    }
}
