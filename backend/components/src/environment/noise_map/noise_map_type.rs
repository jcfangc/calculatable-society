use num_traits::{Float, FromPrimitive};
use serde::Serialize;
use std::fmt;
use std::iter::Sum;

pub trait NoiseMapType:
    From<f32>
    + Send
    + Sync
    + PartialOrd
    + Float
    + Sum
    + FromPrimitive
    + fmt::Display
    + fmt::Debug
    + Serialize
{
}

impl<T> NoiseMapType for T where
    T: From<f32>
        + Send
        + Sync
        + PartialOrd
        + Float
        + Sum
        + FromPrimitive
        + fmt::Display
        + fmt::Debug
        + Serialize
{
}
