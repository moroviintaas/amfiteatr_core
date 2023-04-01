use std::fmt::{Debug, Display};
use crate::action::Action;
use crate::agent::{AgentIdentifier};
use crate::Reward;
use crate::state::State;

pub trait InformationSet: State{
    type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Self::ActionType>;
    type Id: AgentIdentifier;
    type RewardType: Reward;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Self::Id;
    fn is_action_valid(&self, action: &Self::ActionType) -> bool;
    fn current_reward(&self) -> Self::RewardType;
    fn final_reward(&self) -> Option<Self::RewardType>{
        if self.is_finished(){
            Some(self.current_reward())
        } else {
            None
        }
    }

}