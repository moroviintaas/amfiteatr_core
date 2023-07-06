use crate::protocol::DomainParameters;
use crate::{EnvironmentState, State};

pub trait EnvironmentStateUniScore<DP: DomainParameters>: EnvironmentState<DP>{

    fn state_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn penalty_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn add_player_penalty_reward(&mut self, agent: &DP::AgentId, penalty_reward: &DP::UniversalReward);

}