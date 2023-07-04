use crate::protocol::DomainParameters;

pub trait RewardedAgent<DP: DomainParameters>{
    fn current_universal_reward(&self) -> &DP::UniversalReward;
    //fn set_current_universal_reward(&mut self, reward: DP::UniversalReward);
    fn current_universal_reward_add(&mut self, reward_fragment: &DP::UniversalReward);
    fn current_universal_score(&self) -> &DP::UniversalReward;
}