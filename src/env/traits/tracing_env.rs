use crate::agent::Trajectory;
use crate::env::{EnvStateSequential, EnvTrace};
use crate::domain::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvStateSequential<DP>>{

    fn trajectory(&self) -> &Trajectory<EnvTrace<DP, S>>;

}