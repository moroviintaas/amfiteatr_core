use crate::env::{EnvironmentState, StatefulEnvironment};
use crate::domain::DomainParameters;

pub trait ScoreEnvironment<DP: DomainParameters>: StatefulEnvironment<DP>{

    fn process_action_penalise_illegal(
        &mut self,
        agent: &DP::AgentId,
        action: &DP::ActionType,
        penalty_reward: DP::UniversalReward)
        -> Result<<Self::State as EnvironmentState<DP>>::Updates, DP::GameErrorType>;

    fn actual_state_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn actual_penalty_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn actual_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward{
        self.actual_state_score_of_player(agent) + self.actual_penalty_score_of_player(agent)
    }



}