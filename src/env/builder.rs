

use crate::{comm::CommEndpoint};
use crate::env::StatefulEnvironment;

use crate::error::{SetupError};
use crate::domain::{DomainParameters};

pub trait EnvironmentBuilderTrait<DP: DomainParameters, Env: StatefulEnvironment<DP>>: Default{

    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommEndpoint;

    fn build(self) -> Result<Env, SetupError<DP>>;
    fn add_comm(self, agent_id: &DP::AgentId, comm: Self::Comm) -> Result<Self, SetupError<DP>>;
    fn with_state(self, state: Env::State) -> Result<Self, SetupError<DP>>;

}

