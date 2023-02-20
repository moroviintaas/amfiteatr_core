use crate::action::Action;
use crate::state::agent::AgentState;

pub trait Policy{
    type ActionType: Action;
    type StateType: AgentState;

    fn select_action<I: IntoIterator<Item=Self::ActionType>>
        (&self, state: &Self::StateType, actions: I) -> Option<Self::ActionType>;
}