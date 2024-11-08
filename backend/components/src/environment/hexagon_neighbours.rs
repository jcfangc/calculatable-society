use crate::environment::coordinate::Coordinate;
use crate::environment::direction_offset::DirectionOffset;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub NeighbourDirection => DirectionOffset {
        Degree0 => || DirectionOffset { dy: -1, dx: 0 },
        Degree60 => || DirectionOffset { dy: -1, dx: 1 },
        Degree120 => || DirectionOffset { dy: 0, dx: 1 },
        Degree180 => || DirectionOffset { dy: 1, dx: 0 },
        Degree240 => || DirectionOffset { dy: 1, dx: -1 },
        Degree300 => || DirectionOffset { dy: 0, dx: -1 }
    }
}

/// 表示六边形相邻位置的坐标，从正上方（0度）开始顺时针计算
pub struct HexagonNeighbours {
    pub neighbours: HashMap<NeighbourDirection, Coordinate>,
}

impl HexagonNeighbours {
    /// 计算给定坐标在指定方向上的邻居
    fn neighbor_in_direction(
        center: &Coordinate,
        width: usize,
        height: usize,
        direction: NeighbourDirection,
    ) -> Coordinate {
        let direction_offset = NeighbourDirection::to_map()[&direction];
        let new_y = (center.y as isize + direction_offset.dy).rem_euclid(height as isize) as usize;
        let new_x = (center.x as isize + direction_offset.dx).rem_euclid(width as isize) as usize;
        Coordinate::new(new_y, new_x)
    }

    /// 获取六个方向的邻居
    pub fn get_neighbours_for(coordinate: &Coordinate, width: usize, height: usize) -> Self {
        let neighbours = NeighbourDirection::to_map()
            .iter()
            .map(|(&direction, _)| {
                let neighbour = Self::neighbor_in_direction(coordinate, width, height, direction);
                (direction, neighbour)
            })
            .collect::<HashMap<NeighbourDirection, Coordinate>>();

        HexagonNeighbours { neighbours }
    }

    /// 尝试创建一个新的 HexagonNeighbours 实例
    pub fn new(
        neighbour: &Coordinate,
        width: usize,
        height: usize,
        // 对于中心点来说，x 和 y 的方向
        where_center_is_accroding_to_input_neighbour: NeighbourDirection,
    ) -> Self {
        let center = Self::neighbor_in_direction(
            neighbour,
            width,
            height,
            where_center_is_accroding_to_input_neighbour,
        );
        Self::get_neighbours_for(&center, width, height)
    }
}
