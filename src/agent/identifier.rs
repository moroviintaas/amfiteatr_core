use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait AgentIdentifier: Debug + Send + Copy + Hash + Display{

}