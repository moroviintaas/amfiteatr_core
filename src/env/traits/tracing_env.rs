use crate::env::{EnvironmentState, EnvTrajectory};
use crate::domain::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvironmentState<DP>>{

    fn trajectory(&self) -> &EnvTrajectory<DP, S>;

}