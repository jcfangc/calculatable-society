use crate::shared::property::Property;
use crate::shared::property::PropertyCalculator;
use error_handling::SubtanceTypeError;
use num::rational::Ratio;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt;

const LOWER_BOUND: Ratio<usize> = Ratio::new_raw(0, 1);
const UPPER_BOUND: Ratio<usize> = Ratio::new_raw(2, 1);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize)]
pub struct SubtanceType {
    pub subtance_type: Ratio<usize>,
}

impl SubtanceType {
    pub fn try_new(numerator: usize, denominator: usize) -> Result<Self, SubtanceTypeError> {
        if denominator == 0 {
            return Err(SubtanceTypeError::ZeroDenominator);
        }

        let subtance_type = Ratio::new(numerator, denominator);

        if subtance_type < LOWER_BOUND || subtance_type > UPPER_BOUND {
            return Err(SubtanceTypeError::OutOfRange);
        }

        Ok(SubtanceType { subtance_type })
    }
}

impl TryFrom<(usize, usize)> for SubtanceType {
    type Error = SubtanceTypeError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        Self::try_new(value.0, value.1)
    }
}

impl fmt::Display for SubtanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(&self) {
            Ok(json_str) => write!(f, "{}", json_str),
            Err(e) => write!(f, "将 ResourceType 序列化为 JSON 时出错: {}", e),
        }
    }
}

impl PropertyCalculator for SubtanceType {
    async fn calculate(
        &self,
        property: Property,
        frequency_offset: Option<isize>,
        phase_offset: Option<isize>,
    ) -> f64 {
        let param = Property::to_map().get(&property).unwrap();
        let property_value = param
            .with_frequency_offset(frequency_offset.unwrap_or(0))
            .with_phase_offset(phase_offset.unwrap_or(0))
            .calculate(self);
        property_value
    }
}

pub mod error_handling {
    use super::{LOWER_BOUND, UPPER_BOUND};
    use std::fmt;

    #[derive(Debug)]
    pub enum SubtanceTypeError {
        ZeroDenominator,
        OutOfRange,
    }

    impl fmt::Display for SubtanceTypeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SubtanceTypeError::ZeroDenominator => write!(f, "分母不能为零"),
                SubtanceTypeError::OutOfRange => {
                    write!(f, "有效范围：{} - {}", LOWER_BOUND, UPPER_BOUND)
                }
            }
        }
    }
}
