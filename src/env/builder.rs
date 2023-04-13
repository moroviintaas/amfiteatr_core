use crate::{CommEndpoint, CommunicatingEnv, StatefulEnvironment};
use crate::error::SetupError;
use crate::protocol::ProtocolSpecification;

pub trait EnvironmentBuilder: Default{


    type ProtocolSpec: ProtocolSpecification;
    type Environment: CommunicatingEnv + StatefulEnvironment;
    type Comm: CommEndpoint;

    fn build(self) -> Self::Environment;
    fn add_comm(&mut self, comm: Self::Comm) -> Result<(), SetupError<Self::ProtocolSpec>>;
    fn with_state(&mut self, state: <Self::Environment as StatefulEnvironment>::State);

}