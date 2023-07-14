use crate::env::{EnvironmentState, GameHistory};
use crate::protocol::DomainParameters;

pub trait TracingEnv<DP: DomainParameters, S: EnvironmentState<DP>>{

    fn history(&self) -> &GameHistory<DP, S>;

}