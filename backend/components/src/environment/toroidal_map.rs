use std::fmt;

// 地图结构
#[derive(Debug)]
pub struct ToroidalMap {
    width: usize,
    height: usize,
}

impl ToroidalMap {
    // 初始化地图
    pub fn new(width: usize, height: usize) -> Self {
        ToroidalMap { width, height }
    }

    // 返回不可变借用
    pub fn get_ref(&self) -> &Self {
        self
    }

    // 获取宽度和高度的只读访问
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for ToroidalMap {
    fn default() -> Self {
        ToroidalMap::new(1000, 1000)
    }
}

// 打印地图长宽
impl fmt::Display for ToroidalMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToroidalMap({}, {})", self.width, self.height)
    }
}
