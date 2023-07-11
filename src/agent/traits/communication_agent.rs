use std::error::Error;
use crate::agent::Agent;
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};

pub trait CommunicatingAgent<DP: DomainParameters>: Agent<DP>{
    type CommunicationError: Error;

    fn send(&mut self, message: AgentMessage<DP>) -> Result<(), Self::CommunicationError>;
    fn recv(&mut self) -> Result<EnvMessage<DP>, Self::CommunicationError>;
}