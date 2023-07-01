


use crate::protocol::DomainParameters;
use crate::Reward;
use crate::state::State;

pub trait InformationSet<Spec: DomainParameters>: State<Spec>{
    //type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Spec::ActionType>;
    //type Id: AgentIdentifier;
    type RewardType: Reward;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Spec::AgentId;
    fn is_action_valid(&self, action: &Spec::ActionType) -> bool;
    fn current_score(&self) -> Self::RewardType;
    fn final_score(&self) -> Option<Self::RewardType>{
        if self.is_finished(){
            Some(self.current_score())
        } else {
            None
        }
    }

}

impl<T: InformationSet<Spec>, Spec: DomainParameters> InformationSet<Spec> for Box<T> {
    type ActionIteratorType = T::ActionIteratorType;
    type RewardType = T::RewardType;

    fn available_actions(&self) -> Self::ActionIteratorType {
        self.as_ref().available_actions()
    }

    fn id(&self) -> &Spec::AgentId {
        self.as_ref().id()
    }

    fn is_action_valid(&self, action: &Spec::ActionType) -> bool {
        self.as_ref().is_action_valid(action)
    }

    fn current_score(&self) -> Self::RewardType {
        self.as_ref().current_score()
    }
}