use crate::environment::coordinate::Coordinate;
use crate::environment::direction_offset::DirectionOffset;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub DiagonalDirection => DirectionOffset {
        Degree30 => || DirectionOffset { dy: -2, dx: 1 },
        Degree90 => || DirectionOffset { dy: -1, dx: 2 },
        Degree150 => || DirectionOffset { dy: 1, dx: 1 },
        Degree210 => || DirectionOffset { dy: 2, dx: -1 },
        Degree270 => || DirectionOffset { dy: 1, dx: -2 },
        Degree330 => || DirectionOffset { dy: -1, dx: -1 }
    }
}

/// 表示六边形对角线方向的坐标，从30度方向开始顺时针计算
pub struct HexagonDiagonals {
    pub diagonals: HashMap<DiagonalDirection, Coordinate>,
}

impl HexagonDiagonals {
    /// 计算给定坐标在指定方向上的对角
    fn diagonal_in_direction(
        center: &Coordinate,
        width: usize,
        height: usize,
        direction: DiagonalDirection,
    ) -> Coordinate {
        let direction_offset = DiagonalDirection::to_map()[&direction];
        let new_y = (center.y as isize + direction_offset.dy).rem_euclid(height as isize) as usize;
        let new_x = (center.x as isize + direction_offset.dx).rem_euclid(width as isize) as usize;
        Coordinate::new(new_y, new_x)
    }

    /// 获取六个方向的对角
    pub fn get_diagonals_for(coordinate: &Coordinate, width: usize, height: usize) -> Self {
        let diagonals = DiagonalDirection::to_map()
            .iter()
            .map(|(&direction, _)| {
                let neighbour = Self::diagonal_in_direction(coordinate, width, height, direction);
                (direction, neighbour)
            })
            .collect::<HashMap<DiagonalDirection, Coordinate>>();

        HexagonDiagonals { diagonals }
    }

    /// 尝试创建一个新的 HexagonDiagonals 实例
    pub fn new(
        diagonal: &Coordinate,
        width: usize,
        height: usize,
        where_center_is_accroding_to_input_diagonal: DiagonalDirection,
    ) -> Self {
        let center = Self::diagonal_in_direction(
            diagonal,
            width,
            height,
            where_center_is_accroding_to_input_diagonal,
        );
        Self::get_diagonals_for(&center, width, height)
    }
}
