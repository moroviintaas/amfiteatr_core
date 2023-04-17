use std::error::Error;
use crate::agent::AgentIdentifier;
use crate::DomainEnvironment;
use crate::protocol::ProtocolSpecification;

pub trait CommunicatingEnv : DomainEnvironment{
    type Outward;
    type Inward;
    type CommunicationError: Error;

    fn send_to(&mut self, agent_id: &<Self::DomainParameter as ProtocolSpecification>::AgentId,  message: Self::Outward) -> Result<(), Self::CommunicationError>;
    fn recv_from(&mut self, agent_id: &<Self::DomainParameter as ProtocolSpecification>::AgentId) -> Result<Self::Inward, Self::CommunicationError>;

    fn try_recv_from(&mut self, agent_id: &<Self::DomainParameter as ProtocolSpecification>::AgentId) -> Result<Self::Inward, Self::CommunicationError>;


}

pub trait BroadcastingEnv: CommunicatingEnv{

    fn send_to_all(&mut self,  message: Self::Outward) -> Result<(), Self::CommunicationError>;

}