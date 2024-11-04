use crate::shared::property_param::PropertyParam;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub Property => PropertyParam {
        Density => || PropertyParam::new(1, 1, None, None),
        Phase => || PropertyParam::new(2, 2, None, None),
    }
}

// trait Property {
//     fn with_frequency_offset(&self, env_frequency: isize) -> PropertyParam;
//     fn with_phase_offset(&self, env_phase: isize) -> PropertyParam;
//     fn reset_frequency_offset(&self) -> PropertyParam;
//     fn reset_phase_offset(&self) -> PropertyParam;
// }

// pub trait Flammable: Property {
//     fn get_param(&self) -> PropertyParam {
//         self.property_param;
//     }
// }
