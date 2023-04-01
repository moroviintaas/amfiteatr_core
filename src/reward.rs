use std::fmt::Debug;

pub trait Reward: Clone + Debug + PartialEq + Eq + PartialOrd{

}

impl<T: Clone + Debug + PartialEq + Eq + PartialOrd> Reward for T{}