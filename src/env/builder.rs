

use crate::{CommEndpoint, StatefulEnvironment};

use crate::error::{SetupError};
use crate::protocol::{DomainParameters};

pub trait EnvironmentBuilderTrait<Spec: DomainParameters, Env: StatefulEnvironment<Spec>>: Default{

    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommEndpoint;

    fn build(self) -> Result<Env, SetupError<Spec>>;
    fn add_comm(self, agent_id: &Spec::AgentId, comm: Self::Comm) -> Result<Self, SetupError<Spec>>;
    fn with_state(self, state: Env::State) -> Result<Self, SetupError<Spec>>;

}

