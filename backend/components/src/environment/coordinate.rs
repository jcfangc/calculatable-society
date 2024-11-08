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
}
