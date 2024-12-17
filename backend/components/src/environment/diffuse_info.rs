use crate::environment::hexagon::hex_unit::HexUnit;

#[derive(Clone, Debug)]
pub(crate) struct DiffuseInfo {
    unit: HexUnit,
    potential: f64,
}

impl DiffuseInfo {
    pub(crate) fn new(unit: HexUnit, potential: f64) -> Self {
        Self { unit, potential }
    }

    pub(crate) fn unit(&self) -> &HexUnit {
        &self.unit
    }

    pub(crate) fn potential(&self) -> f64 {
        self.potential
    }
}
