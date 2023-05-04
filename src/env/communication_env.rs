use std::error::Error;

use crate::DomainEnvironment;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait CommunicatingEnv<Spec: ProtocolSpecification> : DomainEnvironment<Spec>{
    //type Outward;
    //type Inward;
    type CommunicationError: Error;

    fn send_to(&mut self, agent_id: &Spec::AgentId,  message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError>;
    fn recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError>;

    fn try_recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError>;


}

pub trait BroadcastingEnv<Spec: ProtocolSpecification>: CommunicatingEnv<Spec>{

    fn send_to_all(&mut self,  message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError>;

}