use std::marker::PhantomData;
use crate::state::agent::InformationSet;
use rand::seq::IteratorRandom;

pub trait Policy{
    type StateType: InformationSet;

    fn select_action(&self, state: &Self::StateType) -> Option<<Self::StateType as InformationSet>::ActionType>;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RandomPolicy<State: InformationSet>{
    state: PhantomData<State>
}
impl<State: InformationSet> RandomPolicy<State>{
    pub fn new() -> Self{
        Self{state: PhantomData::default()}
    }
}

impl<State: InformationSet> Policy for RandomPolicy<State>
where <<State as InformationSet>::ActionIteratorType as IntoIterator>::IntoIter : ExactSizeIterator{
    type StateType = State;

    fn select_action(&self, state: &Self::StateType) -> Option<<Self::StateType as InformationSet>::ActionType> {
        let mut rng = rand::thread_rng();
        state.available_actions().into_iter().choose(&mut rng)
    }
}