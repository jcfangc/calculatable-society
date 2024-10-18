use proce_macro::property_model_attribute;

#[property_model_attribute(path = "components/src/shared/property.rs")]
pub struct Property {
    pub numerator: u32,
    pub denominator: u32,
}
