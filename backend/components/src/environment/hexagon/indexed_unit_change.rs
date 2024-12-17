use crate::environment::hexagon::unit_change::UnitChange;
use crate::environment::t_indexed::Indexed;

#[derive(Debug, Clone, Copy)]
pub(crate) struct IndexedUnitChange {
    y: usize,
    x: usize,
    change: UnitChange,
}

impl IndexedUnitChange {
    pub fn new(y: usize, x: usize, change: UnitChange) -> Self {
        Self { y, x, change }
    }

    pub fn change(&self) -> &UnitChange {
        &self.change
    }
}

impl Indexed for IndexedUnitChange {
    fn y(&self) -> usize {
        self.y
    }
    fn x(&self) -> usize {
        self.x
    }
}
