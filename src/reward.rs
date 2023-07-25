use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};

/// `Reward` is trait for types to be used as policy performance assessment.
/// It is implemented for standard types and you can use your own type as reward,
/// if only you made it partially comparable and summable.
pub trait Reward: Send + Clone + Debug + PartialEq  + PartialOrd + Default +
    for<'a> Add<&'a Self, Output=Self> + Add<Output=Self> + for<'a> AddAssign<&'a Self>
    + Sub<Output=Self> + for<'a> Sub<&'a Self, Output=Self> + Sub + Add
{
    /// This is constructor used to produce neutral value of reward, i.e.
    /// the reward that does not change the score. For standard numeric
    /// types this is just value of 0.
    fn neutral() -> Self;
//where for<'a> &'a Self: Add<Output=Self> + Sub<Output=Self>{
//where for<'a> &'a Self: Sub<&'a Self, Output=Self>
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

#[derive(Debug, Copy, Clone)]
pub enum RewardSource{
    Env,
    Agent
}