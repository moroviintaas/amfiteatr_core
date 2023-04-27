use std::collections::HashMap;
use std::fmt::Error;
use crate::{CommEndpoint, CommunicatingEnv, DomainEnvironment, EnvironmentState, GenericEnvironment, StatefulEnvironment};
use crate::automatons::rr::EnvironmentRR;
use crate::error::{CommError, SetupError};
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait EnvironmentBuilderTrait<Spec: ProtocolSpecification, Env: StatefulEnvironment<Spec>>: Default{

    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommEndpoint;

    fn build(self) -> Result<Env, SetupError<Spec>>;
    fn add_comm(self, agent_id: &Spec::AgentId, comm: Self::Comm) -> Result<Self, SetupError<Spec>>;
    fn with_state(self, state: Env::State) -> Result<Self, SetupError<Spec>>;

}

