


use crate::protocol::DomainParameters;
use crate::Reward;
use crate::state::State;

pub trait InformationSet<Spec: DomainParameters>: State<Spec>{
    type ActionIteratorType: IntoIterator<Item = Spec::ActionType>;



    fn available_actions(&self) -> Self::ActionIteratorType;
    fn is_action_valid(&self, action: &Spec::ActionType) -> bool;


}

pub trait ScoringInformationSet<DP: DomainParameters>: InformationSet<DP>{
    type RewardType: Reward;
    fn current_subjective_score(&self) -> Self::RewardType;
}

impl<T: InformationSet<Spec>, Spec: DomainParameters> InformationSet<Spec> for Box<T> {
    type ActionIteratorType = T::ActionIteratorType;


    fn available_actions(&self) -> Self::ActionIteratorType {
        self.as_ref().available_actions()
    }


    fn is_action_valid(&self, action: &Spec::ActionType) -> bool {
        self.as_ref().is_action_valid(action)
    }


}

impl<T: ScoringInformationSet<Spec>, Spec: DomainParameters> ScoringInformationSet<Spec> for Box<T> {
    type RewardType = T::RewardType;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.as_ref().current_subjective_score()
    }
}