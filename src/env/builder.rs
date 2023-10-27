

use crate::{comm::CommPort};
use crate::env::StatefulEnvironment;

use crate::domain::{DomainParameters};
use crate::error::WorldError;

pub trait EnvironmentBuilderTrait<DP: DomainParameters, Env: StatefulEnvironment<DP>>: Default{

    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommPort;

    fn build(self) -> Result<Env, WorldError<DP>>;
    fn add_comm(self, agent_id: &DP::AgentId, comm: Self::Comm) -> Result<Self, WorldError<DP>>;
    fn with_state(self, state: Env::State) -> Result<Self, WorldError<DP>>;

}

