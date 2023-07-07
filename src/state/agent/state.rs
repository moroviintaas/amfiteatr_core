


use crate::protocol::DomainParameters;
use crate::Reward;
use crate::state::State;

pub trait InformationSet<Spec: DomainParameters>: State<Spec>{
    //type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Spec::ActionType>;
    //type Id: AgentIdentifier;


    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Spec::AgentId;
    fn is_action_valid(&self, action: &Spec::ActionType) -> bool;


}

pub trait ScoringInformationSet<DP: DomainParameters>: InformationSet<DP>{
    type RewardType: Reward;
    fn current_subjective_score(&self) -> Self::RewardType;
    fn final_subjective_score(&self) -> Option<Self::RewardType>{
        if self.is_finished(){
            Some(self.current_subjective_score())
        } else {
            None
        }
    }
}

impl<T: InformationSet<Spec>, Spec: DomainParameters> InformationSet<Spec> for Box<T> {
    type ActionIteratorType = T::ActionIteratorType;


    fn available_actions(&self) -> Self::ActionIteratorType {
        self.as_ref().available_actions()
    }

    fn id(&self) -> &Spec::AgentId {
        self.as_ref().id()
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