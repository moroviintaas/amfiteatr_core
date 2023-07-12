use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait AgentIdentifier: Debug + Send + Copy + Hash + Display + PartialEq + Eq{

}

macro_rules! impl_agent_id_std {
    ($($x: ty), +) => {
        $(
          impl AgentIdentifier for $x{}

        )*

    }
}

impl_agent_id_std!(u8, u16, u32, u64, u128);