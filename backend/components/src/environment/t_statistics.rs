pub trait Statistics {
    type Item;
    fn min(&self) -> Self::Item;
    fn max(&self) -> Self::Item;
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
}
