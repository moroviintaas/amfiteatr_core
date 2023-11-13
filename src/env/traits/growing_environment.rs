use crate::{comm::CommPort};
use crate::env::CommunicatingEnv;
use crate::error::{CommunicationError, WorldError};
use crate::domain::{AgentMessage, EnvMessage, DomainParameters};

pub trait ExtendableEnv<DP: DomainParameters>: CommunicatingEnv<DP>{
    type Port: CommPort<OutwardType=EnvMessage<DP>, InwardType=AgentMessage<DP>, Error=CommunicationError<DP>> + ?Sized;
    fn connect_agent(&mut self, agent_id: DP::AgentId, endpoint: Self::Port) -> Result<(), WorldError<DP>>;

}