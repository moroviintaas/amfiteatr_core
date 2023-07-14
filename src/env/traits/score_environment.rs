use crate::env::StatefulEnvironment;
use crate::protocol::DomainParameters;

pub trait ScoreEnvironment<DP: DomainParameters>: StatefulEnvironment<DP>{

    fn process_action_penalise_illegal(
        &mut self,
        agent: &DP::AgentId,
        action: &DP::ActionType,
        penalty_reward: DP::UniversalReward)
        -> Result<Self::UpdatesIterator, DP::GameErrorType>;

    fn actual_state_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn actual_penalty_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;
    fn actual_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward{
        self.actual_state_score_of_player(agent) + self.actual_penalty_score_of_player(agent)
    }



}