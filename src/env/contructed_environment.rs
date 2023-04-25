use std::collections::HashMap;
use crate::{CommEndpoint, EnvCommEndpoint, EnvironmentState, StatefulEnvironment};
use crate::error::SetupError;
use crate::protocol::ProtocolSpecification;

pub trait ConstructedEnvironment<Spec: ProtocolSpecification,  EnvComm: CommEndpoint>: Sized + StatefulEnvironment<Spec>{

    fn construct(state: Self::State, env_comms: HashMap<Spec::AgentId, EnvComm>) -> Result<Self, SetupError<Spec>>;

}