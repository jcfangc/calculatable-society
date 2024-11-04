use crate::shared::property_param::PropertyParam;
use crate::shared::subtance_type::SubtanceType;
use std::collections::HashSet;

pub struct Subtance {
    subtance_type: SubtanceType,
    properties: HashSet<PropertyParam>,
}
