use std::fmt::Error;
use crate::{CommEndpoint, CommunicatingEnv, DomainEnvironment, StatefulEnvironment};
use crate::automatons::rr::EnvironmentRR;
use crate::error::{CommError, SetupError};
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait EnvironmentBuilder: Default{


    type ProtocolSpec: ProtocolSpecification;
    type Environment: CommunicatingEnv<Outward = EnvMessage<Self::ProtocolSpec>, Inward = AgentMessage<Self::ProtocolSpec>, CommunicationError = CommError> + StatefulEnvironment;
    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommEndpoint;

    fn build(self) -> Self::Environment;
    fn add_comm(self, agent_id: &<<Self::Environment as DomainEnvironment>::DomainParameter as ProtocolSpecification>::AgentId, comm: Self::Comm) -> Result<Self, SetupError<Self::ProtocolSpec>>;
    fn with_state(self, state: <Self::Environment as StatefulEnvironment>::State) -> Result<Self, SetupError<Self::ProtocolSpec>>;

}