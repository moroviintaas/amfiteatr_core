use crate::agent::StatefulAgent;
use crate::domain::DomainParameters;
use crate::state::agent::ScoringInformationSet;



/// Represents agent who is able to produce subjective score based on it's
/// _information set_. This is ability available only for  using information sets that can be evaluated to rewards.
pub trait InternalRewardedAgent<DP: DomainParameters>: StatefulAgent<DP>
where <Self as StatefulAgent<DP>>::State: ScoringInformationSet<DP>{
    /// Returns current subjective score which should be sum of information set's internal score
    /// record/evaluation and explicit subjective component stored by agent.
    ///
    fn current_subjective_score(&self) -> <<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType;
    /// Add explicit component of internal state. Typically it could be some penalty
    /// > e.g. when agent tried to perform illegal operation, state of game is not
    /// mutated forward and information set does not change however for reinforcement learning
    /// it will be useful to note that this particular action is not be taken in this situation.
    fn add_explicit_subjective_score(&mut self, explicit_reward: &<<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType);
}