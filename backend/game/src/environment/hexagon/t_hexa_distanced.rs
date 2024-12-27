pub(crate) trait HexaDistanced {
    fn z(&self) -> isize;
    fn distance_to(&self, other: &Self) -> usize;
}
