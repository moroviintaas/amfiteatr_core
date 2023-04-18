use std::error::Error;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait CommunicatingAgent<Spec: ProtocolSpecification>{
    type CommunicationError: Error;

    fn send(&mut self, message: AgentMessage<Spec>) -> Result<(), Self::CommunicationError>;
    fn recv(&mut self) -> Result<EnvMessage<Spec>, Self::CommunicationError>;
}