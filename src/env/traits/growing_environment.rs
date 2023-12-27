use crate::{comm::BidirectionalEndpoint};
use crate::env::CommunicatingEnv;
use crate::error::{CommunicationError, WorldError};
use crate::domain::{AgentMessage, EnvironmentMessage, DomainParameters};

pub trait ExtendableEnv<DP: DomainParameters>: CommunicatingEnv<DP>{
    type Endpoint: BidirectionalEndpoint<OutwardType=EnvironmentMessage<DP>, InwardType=AgentMessage<DP>, Error=CommunicationError<DP>> + ?Sized;
    fn connect_agent(&mut self, agent_id: DP::AgentId, endpoint: Self::Endpoint) -> Result<(), WorldError<DP>>;

}