use std::fmt::{Debug, Display};
use crate::action::Action;
use crate::agent::{AgentIdentifier};
use crate::state::State;

pub trait InformationSet: State{
    type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Self::ActionType>;
    type Id: AgentIdentifier;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Self::Id;
    fn is_action_valid(&self, action: &Self::ActionType) -> bool;

}