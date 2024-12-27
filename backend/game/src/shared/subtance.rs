use crate::shared::property_param::PropertyParam;
use crate::shared::subtance_type::SubstanceType;
use std::collections::HashSet;

pub(crate) struct Subtance {
    subtance_type: SubstanceType,
    properties: HashSet<PropertyParam>,
}
