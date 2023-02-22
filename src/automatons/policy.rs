use crate::action::Action;
use crate::state::agent::AgentState;

pub trait Policy{
    type StateType: AgentState;

    fn select_action(&self, state: &Self::StateType) -> Option<<Self::StateType as AgentState>::ActionType>;
}