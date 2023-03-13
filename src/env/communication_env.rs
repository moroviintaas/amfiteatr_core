use std::error::Error;
use crate::agent::AgentIdentifier;

pub trait CommunicatingEnv{
    type Outward;
    type Inward;
    type CommunicationError: Error;
    type AgentId: AgentIdentifier;

    fn send_to(&mut self, agent_id: &Self::AgentId,  message: Self::Outward) -> Result<(), Self::CommunicationError>;
    fn recv_from(&mut self, agent_id: &Self::AgentId) -> Result<Self::Inward, Self::CommunicationError>;

    fn try_recv_from(&mut self, agent_id: &Self::AgentId) -> Result<Self::Inward, Self::CommunicationError>;


}

pub trait BroadcastingEnv: CommunicatingEnv{

    fn send_to_all(&mut self,  message: Self::Outward) -> Result<(), Self::CommunicationError>;

}