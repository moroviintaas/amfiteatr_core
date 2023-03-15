use std::marker::PhantomData;
use crate::state::agent::AgentState;
use rand::seq::IteratorRandom;

pub trait Policy{
    type StateType: AgentState;

    fn select_action(&self, state: &Self::StateType) -> Option<<Self::StateType as AgentState>::ActionType>;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RandomPolicy<State: AgentState>{
    state: PhantomData<State>
}
impl<State: AgentState> RandomPolicy<State>{
    pub fn new() -> Self{
        Self{state: PhantomData::default()}
    }
}

impl<State: AgentState> Policy for RandomPolicy<State>
where <<State as AgentState>::ActionIteratorType as IntoIterator>::IntoIter : ExactSizeIterator{
    type StateType = State;

    fn select_action(&self, state: &Self::StateType) -> Option<<Self::StateType as AgentState>::ActionType> {
        let mut rng = rand::thread_rng();
        state.available_actions().into_iter().choose(&mut rng)
    }
}