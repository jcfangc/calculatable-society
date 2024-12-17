use crate::environment::coordinate::Coordinate;

pub(crate) trait Indexed {
    fn y(&self) -> usize;
    fn x(&self) -> usize;

    fn coordinate(&self) -> Coordinate {
        Coordinate::new(self.y(), self.x())
    }
}
