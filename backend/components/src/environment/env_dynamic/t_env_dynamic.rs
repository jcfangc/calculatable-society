pub trait EnvDynamic {
    type StateType;

    fn update(&mut self);
    fn state(&self) -> &Self::StateType;
}
