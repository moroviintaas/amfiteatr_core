use crate::agent::Trajectory;
use crate::env::{EnvironmentStateSequential, EnvironmentTraceStep};
use crate::domain::DomainParameters;


/// Environment that provide tracing game.
pub trait TracingEnv<DP: DomainParameters, S: EnvironmentStateSequential<DP>>{

    
    fn trajectory(&self) -> &Trajectory<EnvironmentTraceStep<DP, S>>;

}