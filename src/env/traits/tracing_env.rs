use crate::agent::AgentTrajectory;
use crate::env::{EnvStateSequential, EnvTrace};
use crate::domain::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvStateSequential<DP>>{

    fn trajectory(&self) -> &AgentTrajectory<EnvTrace<DP, S>>;

}