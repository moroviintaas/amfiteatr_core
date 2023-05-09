use std::fmt::Debug;
use std::ops::{Add, Sub};

pub trait Reward: Clone + Debug + PartialEq + Eq + PartialOrd + Default + Add<Output=Self> + Sub<Output=Self>
{
//where for<'a> &'a Self: Add<Output=Self> + Sub<Output=Self>{

}

impl<T: Clone + Debug + PartialEq + Eq + PartialOrd + Default + Add<Output=Self> + Sub<Output=Self>> Reward for T {}