
use crate::{comm::CommPort};
use crate::env::StatefulEnvironment;

use crate::domain::DomainParameters;

pub trait ConstructedEnvironment<DP: DomainParameters,  EnvComm: CommPort>: Sized + StatefulEnvironment<DP>{

    //fn construct(state: Self::State, env_comms: HashMap<Spec::AgentId, EnvComm>) -> Result<Self, SetupError<Spec>>;

}