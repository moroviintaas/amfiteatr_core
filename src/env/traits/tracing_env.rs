use crate::env::{EnvStateSequential, EnvTrajectory};
use crate::domain::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvStateSequential<DP>>{

    fn trajectory(&self) -> &EnvTrajectory<DP, S>;

}