use std::fmt::Debug;
use std::ops::{Add, Sub};

pub trait Reward: Send + Clone + Debug + PartialEq + Eq + PartialOrd + Default + Add<Output=Self> + Sub<Output=Self>
{

    fn neutral() -> Self;
//where for<'a> &'a Self: Add<Output=Self> + Sub<Output=Self>{

}

impl<T: Send + Clone + Debug + PartialEq + Eq + PartialOrd + Default + Add<Output=Self> + Sub<Output=Self> + Default> Reward for T {
    fn neutral() -> Self {
        T::default()
    }
}