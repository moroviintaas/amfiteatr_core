use crate::agent::{Agent, AgentTrajectory};
use crate::domain::DomainParameters;
use crate::state::agent::{ScoringInformationSet};

pub trait TracingAgent<DP: DomainParameters, S: ScoringInformationSet<DP>>: Agent<DP>{
    fn reset_trajectory(&mut self);
    fn take_trajectory(&mut self) -> AgentTrajectory<DP, S>;
    //fn set_new_state(&mut self);
    fn game_trajectory(&self) -> &AgentTrajectory<DP, S>;
    fn commit_trace(&mut self);
    fn explicit_add_subjective_reward(&mut self, explicit: S::RewardType);
    //fn mark_last_action_illegal(&mut self);

}