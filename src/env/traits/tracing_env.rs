use crate::env::{EnvStateSequential, EnvTrace, GameTrajectory};
use crate::domain::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvStateSequential<DP>>{

    fn trajectory(&self) -> &GameTrajectory<EnvTrace<DP, S>>;

}