pub trait HexaDistanced {
    fn z(&self) -> isize;
    fn distance(&self, other: &Self) -> usize;
}
