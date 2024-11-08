use crate::shared::property_param::PropertyParam;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub Property => PropertyParam {
        Density => || PropertyParam::new(1, 1, None, None),
        Phase => || PropertyParam::new(2, 2, None, None),
    }
}

pub trait PropertyCalculator {
    async fn calculate(
        &self,
        property: Property,
        frequency_offset: Option<isize>,
        phase_offset: Option<isize>,
    ) -> f64;
}
