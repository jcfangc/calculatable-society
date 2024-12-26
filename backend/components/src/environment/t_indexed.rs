use crate::environment::hexagon::hex_coord::HexCoord;

pub(crate) trait Indexed {
    fn y(&self) -> usize;
    fn x(&self) -> usize;

    fn coordinate(&self) -> HexCoord {
        HexCoord::new(self.y(), self.x())
    }
}
