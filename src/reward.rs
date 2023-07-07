use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};

pub trait Reward: Send + Clone + Debug + PartialEq  + PartialOrd + Default +
    for<'a> Add<&'a Self, Output=Self> + Add<Output=Self> + for<'a> AddAssign<&'a Self>
    + Sub<Output=Self> + for<'a> Sub<&'a Self, Output=Self>
{

    fn neutral() -> Self;
//where for<'a> &'a Self: Add<Output=Self> + Sub<Output=Self>{

}
/*
impl<T: Send + Clone + Debug + PartialEq + Eq + PartialOrd + Default +
    for<'a> Add<&'a Self, Output=Self> + Add<Output=Self>  + for<'a> AddAssign<&'a Self>
    + Sub<Output=Self> + for<'a> Sub<&'a Self, Output=Self>
    + Default> Reward for T {
    fn neutral() -> Self {
        T::default()
    }
}*/

macro_rules! impl_reward_std {
    ($($x: ty), +) => {
        $(
          impl Reward for $x{
              fn neutral() -> $x{
                  0
              }
          }

        )*

    }
}

impl_reward_std![u8, u16, u32, u64, i8, i16, i32, i64];

impl Reward for f32{
    fn neutral() -> Self {
        0.0
    }
}
impl Reward for f64{
    fn neutral() -> Self {
        0.0
    }
}