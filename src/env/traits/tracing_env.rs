use crate::env::{EnvironmentState, EnvHistory};
use crate::protocol::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvironmentState<DP>>{

    fn trajectory(&self) -> &EnvHistory<DP, S>;

}