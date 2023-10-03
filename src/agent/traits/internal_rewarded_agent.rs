use crate::agent::StatefulAgent;
use crate::domain::DomainParameters;
use crate::state::agent::ScoringInformationSet;

pub trait InternalRewardedAgent<DP: DomainParameters>: StatefulAgent<DP>
where <Self as StatefulAgent<DP>>::State: ScoringInformationSet<DP>{
    fn current_subjective_score(&self) -> <<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType;
    fn add_explicit_subjective_score(&mut self, explicit_reward: &<<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType);
}