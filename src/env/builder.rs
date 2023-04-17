use crate::{CommEndpoint, CommunicatingEnv, DomainEnvironment, StatefulEnvironment};
use crate::error::SetupError;
use crate::protocol::ProtocolSpecification;

pub trait EnvironmentBuilder: Default{


    type ProtocolSpec: ProtocolSpecification;
    type Environment: CommunicatingEnv + StatefulEnvironment;
    type Comm: CommEndpoint;

    fn build(self) -> Self::Environment;
    fn add_comm(self, agent_id: &<<Self::Environment as DomainEnvironment>::DomainParameter as ProtocolSpecification>::AgentId, comm: Self::Comm) -> Result<Self, SetupError<Self::ProtocolSpec>>;
    fn with_state(self, state: <Self::Environment as StatefulEnvironment>::State) -> Result<Self, SetupError<Self::ProtocolSpec>>;

}