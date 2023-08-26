


use crate::protocol::DomainParameters;
use crate::Reward;
use crate::state::ConstructedState;

pub trait InformationSet<DP: DomainParameters>: Send{
    type ActionIteratorType: IntoIterator<Item = DP::ActionType>;



    fn available_actions(&self) -> Self::ActionIteratorType;
    fn is_action_valid(&self, action: &DP::ActionType) -> bool;
    fn update(&mut self, update: DP::UpdateType) -> Result<(), DP::GameErrorType>;


}

impl<DP: DomainParameters, T: InformationSet<DP>> InformationSet<DP> for Box<T>{
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
pub trait ScoringInformationSet<DP: DomainParameters>: InformationSet<DP>{
    type RewardType: Reward;
    fn current_subjective_score(&self) -> Self::RewardType;
    fn penalty_for_illegal() -> Self::RewardType;
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

pub trait ConstructedInfoSet<DP: DomainParameters, B>: InformationSet<DP> + ConstructedState<DP, B> {}
impl<DP: DomainParameters, B, T: InformationSet<DP> + ConstructedState<DP, B>> ConstructedInfoSet<DP, B> for T{}

//impl<DP: DomainParameters, B, T: ConstructedInfoSet<DP, B>> ConstructedInfoSet<DP, B> for Box<T>{}