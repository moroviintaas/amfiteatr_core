use crate::action::Action;
use crate::agent::{AgentIdentifier, IdentifiableAgent};
use crate::state::State;

pub trait AgentState : State{
    type ActionType: Action;
    type ActionIteratorType: IntoIterator<Item = Self::ActionType>;
    type Id: AgentIdentifier;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Self::Id;

}