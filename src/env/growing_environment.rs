use crate::{CommEndpoint, CommunicatingEnv};
use crate::error::{CommError, SetupError};
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait GrowingEnvironment<Spec: ProtocolSpecification>: CommunicatingEnv<Spec>{
    type Endpoint: CommEndpoint<OutwardType=EnvMessage<Spec>, InwardType=AgentMessage<Spec>, Error=CommError<Spec>> + ?Sized;
    fn add_connection(&mut self, agent_id: Spec::AgentId, endpoint: Self::Endpoint) -> Result<(), SetupError<Spec>>;

}