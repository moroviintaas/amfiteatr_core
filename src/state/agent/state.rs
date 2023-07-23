


use crate::protocol::DomainParameters;
use crate::Reward;

pub trait InformationSet<DP: DomainParameters>: Clone + Send{
    type ActionIteratorType: IntoIterator<Item = DP::ActionType>;



    fn available_actions(&self) -> Self::ActionIteratorType;
    fn is_action_valid(&self, action: &DP::ActionType) -> bool;
    fn update(&mut self, update: DP::UpdateType) -> Result<(), DP::GameErrorType>;


}

pub trait ScoringInformationSet<DP: DomainParameters>: InformationSet<DP>{
    type RewardType: Reward;
    fn current_subjective_score(&self) -> Self::RewardType;
    fn penalty_for_illegal() -> Self::RewardType;
}

impl<T: InformationSet<DP>, DP: DomainParameters> InformationSet<DP> for Box<T> {
    type ActionIteratorType = T::ActionIteratorType;


    fn available_actions(&self) -> Self::ActionIteratorType {
        self.as_ref().available_actions()
    }


    fn is_action_valid(&self, action: &DP::ActionType) -> bool {
        self.as_ref().is_action_valid(action)
    }

    fn update(&mut self, update: DP::UpdateType) -> Result<(), DP::GameErrorType> {
        self.as_mut().update(update)
    }

}

impl<T: ScoringInformationSet<Spec>, Spec: DomainParameters> ScoringInformationSet<Spec> for Box<T> {
    type RewardType = T::RewardType;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.as_ref().current_subjective_score()
    }

    fn penalty_for_illegal() -> T::RewardType {
        T::penalty_for_illegal()
    }
}