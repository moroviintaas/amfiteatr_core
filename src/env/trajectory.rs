use std::fmt::{Display, Formatter};
pub use crate::agent::Trajectory;
use crate::env::EnvironmentStateSequential;
use crate::domain::DomainParameters;


/// Trace step of while game (traced by game environment)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct EnvironmentTraceStep<DP: DomainParameters, S: EnvironmentStateSequential<DP>>{
    state_before: S,
    agent: DP::AgentId,
    action: DP::ActionType,
    is_action_valid: bool
}

impl<DP: DomainParameters, S: EnvironmentStateSequential<DP>> Display for EnvironmentTraceStep<DP, S>
where S: Display, <DP as DomainParameters>::AgentId: Display,
      <DP as DomainParameters>::ActionType: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ][ {} / {} ", self.state_before, self.agent, self.action)?;
        if !self.is_action_valid{
            write!(f, "!]")
        } else {
            write!(f, "]")
        }
    }
}


impl<DP: DomainParameters, S: EnvironmentStateSequential<DP>> EnvironmentTraceStep<DP, S>{

    pub fn new(state_before: S, agent: DP::AgentId,
               action: DP::ActionType, is_valid: bool) -> Self{
        /*let checked_action = match is_valid{
            false => CheckedAction::Invalid(action),
            true => CheckedAction::Valid(action)
        };

         */
        Self{state_before, agent, action, is_action_valid: is_valid}
    }

    pub fn state_before(&self) -> &S{
        &self.state_before
    }

    pub fn agent(&self) -> &DP::AgentId{
        &self.agent
    }

    pub fn action(&self) -> &DP::ActionType{
        &self.action
    }


    pub fn is_action_valid(&self) -> bool{
        self.is_action_valid
    }
}

/// Standard trajectory for environment
pub type StdEnvironmentTrajectory<DP, S> = Trajectory<EnvironmentTraceStep<DP, S>>;
