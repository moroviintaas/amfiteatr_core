
use crate::{CommEndpoint, StatefulEnvironment};

use crate::protocol::DomainParameters;

pub trait ConstructedEnvironment<Spec: DomainParameters,  EnvComm: CommEndpoint>: Sized + StatefulEnvironment<Spec>{

    //fn construct(state: Self::State, env_comms: HashMap<Spec::AgentId, EnvComm>) -> Result<Self, SetupError<Spec>>;

}