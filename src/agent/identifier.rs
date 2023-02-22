use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait AgentIdentifier: Debug + Send + Copy + Hash + Display{

}

pub trait IdentifiableAgent{
    type Id: AgentIdentifier;

    fn id(&self) -> &Self::Id;
}