use crate::_commute::t_from_dto::FromDTO;
use crate::shared::property_param::PropertyParam;
use backend_core::dto::dto_property_params::DTOPropertyParams;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub Property => PropertyParam {
        Density => || PropertyParam::new(1, 1, None, None),
        Phase => || PropertyParam::new(2, 2, None, None),
    }
}

impl FromDTO<DTOPropertyParams> for Property {
    fn from_dto(dto: DTOPropertyParams) -> Self {
        match Property::to_map().iter().find(|(_, v)| {
            v.frequency_constant == dto.frequency_constant && v.phase_constant == dto.phase_constant
        }) {
            Some((property, _)) => {
                tracing::trace!(
                    "属性查找成功: frequency_constant = {}, phase_constant = {}",
                    dto.frequency_constant,
                    dto.phase_constant
                );
                *property
            }
            None => {
                let fail_message = format!(
                    "未找到属性: frequency_constant = {}, phase_constant = {}",
                    dto.frequency_constant, dto.phase_constant
                );
                tracing::error!(fail_message);
                panic!("{}", fail_message);
            }
        }
    }
}
