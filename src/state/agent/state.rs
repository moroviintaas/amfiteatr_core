use crate::action::Action;
use crate::state::State;

pub trait AgentState : State{
    type ActionType: Action;
    type ActionIterator: IntoIterator<Item = Self::ActionType>;

    fn available_actions(&self) -> Self::ActionIterator;
}