use std::error::Error;
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};

pub trait CommunicatingAgent<Spec: DomainParameters>{
    type CommunicationError: Error;

    fn send(&mut self, message: AgentMessage<Spec>) -> Result<(), Self::CommunicationError>;
    fn recv(&mut self) -> Result<EnvMessage<Spec>, Self::CommunicationError>;
}