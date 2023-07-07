use crate::env::EnvironmentState;
use crate::protocol::DomainParameters;

pub trait EnvironmentStateUniScore<DP: DomainParameters>: EnvironmentState<DP>{

    fn state_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;

}