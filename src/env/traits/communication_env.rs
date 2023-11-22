use std::error::Error;

use crate::{domain::{AgentMessage, EnvMessage, DomainParameters}, error::CommunicationError};

pub trait CommunicatingEnv<Spec: DomainParameters>{
    //type Outward;
    //type Inward;
    type CommunicationError: Error;

    fn send_to(&mut self, agent_id: &Spec::AgentId,  message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError>;
    fn recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError>;

    fn try_recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<Option<AgentMessage<Spec>>, Self::CommunicationError>;


}

pub trait BroadcastingEnv<Spec: DomainParameters>: CommunicatingEnv<Spec>{

    fn send_to_all(&mut self,  message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError>;

}

pub trait ConnectedEnvironment<DP: DomainParameters>{
    
    fn send(&mut self, agent_id: &DP::AgentId,  message: EnvMessage<DP>)
        -> Result<(), CommunicationError<DP>>;
    fn receive_blocking(&mut self)
        -> Result<(DP::AgentId, AgentMessage<DP>), CommunicationError<DP>>;
    fn receive_nonblocking(&mut self)
        -> Result<Option<(DP::AgentId, AgentMessage<DP>)>, CommunicationError<DP>>;
        
}

pub trait BroadConnectedEnvironment<DP: DomainParameters>{
    fn send_all(&mut self, message: EnvMessage<DP>) -> Result<(), CommunicationError<DP>>;
}