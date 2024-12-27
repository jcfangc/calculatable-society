use crate::_commute::t_from_dto::FromDTO;
use crate::shared::property_param::PropertyParam;
use crate::shared::subtance_type::SubstanceType;
use back_core::dto::dto_property_params::DTOPropertyParams;
use utils::enum_map;

enum_map! {
    #[derive(Clone, Copy)]
    pub(crate) Property => PropertyParam {
        // 摩尔质量
        MolarMass => || PropertyParam::new(1, 1, None, None),
        // 密度
        Density => || PropertyParam::new(2, 2, None, None),
        // 流动性
        Fluidity => || PropertyParam::new(3, 3, None, None),
    }
}

impl Property {
    /// 获取指定属性值
    ///
    /// ### 参数
    /// - `property`: `Property` 类型的枚举值，表示要获取的属性。
    /// - `substance_type`: `SubstanceType`，物质类型。
    ///
    /// ### 返回值
    /// 返回计算后的属性值。
    pub(crate) fn calculate_property(property: Property, substance_type: &SubstanceType) -> f64 {
        // 从静态映射中获取对应的属性参数
        Property::to_map()
            .get(&property)
            .expect("属性未找到")
            .calculate(substance_type)
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
