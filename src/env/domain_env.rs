use crate::protocol::ProtocolSpecification;

pub trait DomainEnvironment{
    type DomainParameter: ProtocolSpecification;
}