use crate::environment::coordinate_shift::CoordinateShift;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, Copy)]
pub struct SubstanceDistributionState {
    amount: usize,
    remain_negative_gradient: CoordinateShift,
}

impl SubstanceDistributionState {
    pub fn new(amount: usize, remain_negative_gradient: CoordinateShift) -> Self {
        Self {
            amount,
            remain_negative_gradient: remain_negative_gradient,
        }
    }

    pub fn amount(&self) -> usize {
        self.amount
    }

    pub fn remain_negative_gradient(&self) -> CoordinateShift {
        self.remain_negative_gradient
    }

    pub fn set_amount(&mut self, amount: usize) {
        self.amount = amount;
    }

    pub fn set_remain_negative_gradient(&mut self, remain_negative_gradient: CoordinateShift) {
        self.remain_negative_gradient = remain_negative_gradient;
    }
}

impl Default for SubstanceDistributionState {
    fn default() -> Self {
        Self {
            amount: 0,
            remain_negative_gradient: CoordinateShift::new(0, 0),
        }
    }
}
