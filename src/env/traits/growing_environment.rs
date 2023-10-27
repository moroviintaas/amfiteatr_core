use crate::{comm::CommPort};
use crate::env::CommunicatingEnv;
use crate::error::{CommunicationError, WorldError};
use crate::domain::{AgentMessage, EnvMessage, DomainParameters};

pub trait GrowingEnvironment<Spec: DomainParameters>: CommunicatingEnv<Spec>{
    type Endpoint: CommPort<OutwardType=EnvMessage<Spec>, InwardType=AgentMessage<Spec>, Error=CommunicationError<Spec>> + ?Sized;
    fn add_connection(&mut self, agent_id: Spec::AgentId, endpoint: Self::Endpoint) -> Result<(), WorldError<Spec>>;

}