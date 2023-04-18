use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::protocol::ProtocolSpecification;

pub trait AgentIdentifier: Debug + Send + Copy + Hash + Display + PartialEq + Eq{

}

pub trait DistinctAgent<Spec: ProtocolSpecification> {
    //type Id: AgentIdentifier;

    fn id(&self) -> &Spec::AgentId;
}