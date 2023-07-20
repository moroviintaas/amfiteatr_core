use crate::env::{EnvironmentState, EnvTrajectory};
use crate::protocol::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvironmentState<DP>>{

    fn trajectory(&self) -> &EnvTrajectory<DP, S>;

}