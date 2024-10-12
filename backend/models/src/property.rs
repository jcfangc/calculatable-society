use macros::property_model_attribute;
// use sqlx::FromRow;

#[property_model_attribute(path = "components/src/shared/property.rs")]
// #[derive(Debug, FromRow)]
pub struct PropertyModel {
    pub numerator: u32,
    pub denominator: u32,
}
