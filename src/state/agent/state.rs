use std::fmt::{Debug, Display};
use crate::action::Action;
use crate::agent::{AgentIdentifier};
use crate::protocol::ProtocolSpecification;
use crate::Reward;
use crate::state::State;

pub trait InformationSet<Spec: ProtocolSpecification>: State<Spec>{
    //type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Spec::ActionType>;
    //type Id: AgentIdentifier;
    type RewardType: Reward;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Spec::AgentId;
    fn is_action_valid(&self, action: &Spec::ActionType) -> bool;
    fn current_reward(&self) -> Self::RewardType;
    fn final_reward(&self) -> Option<Self::RewardType>{
        if self.is_finished(){
            Some(self.current_reward())
        } else {
            None
        }
    }

}