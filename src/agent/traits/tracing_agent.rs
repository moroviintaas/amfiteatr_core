use crate::agent::{Agent, AgentTrajectory};
use crate::protocol::DomainParameters;
use crate::state::agent::{ScoringInformationSet};

pub trait TracingAgent<DP: DomainParameters, S: ScoringInformationSet<DP>>: Agent<DP>{
    fn reset_trace(&mut self);
    //fn set_new_state(&mut self);
    fn game_trajectory(&self) -> &AgentTrajectory<DP, S>;
    fn commit_trace(&mut self);

}