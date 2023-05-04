use std::marker::PhantomData;
use crate::state::agent::InformationSet;
use rand::seq::IteratorRandom;
use crate::protocol::ProtocolSpecification;


pub trait Policy<Spec: ProtocolSpecification>{
    type StateType: InformationSet<Spec>;

    fn select_action(&self, state: &Self::StateType) -> Option<Spec::ActionType>;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RandomPolicy<Spec: ProtocolSpecification, State: InformationSet<Spec>>{
    state: PhantomData<State>,
    _spec: PhantomData<Spec>
}
impl<Spec: ProtocolSpecification, State: InformationSet<Spec>> RandomPolicy<Spec, State>{
    pub fn new() -> Self{
        Self{state: PhantomData::default(), _spec: PhantomData::default()}
    }
}

impl<Spec: ProtocolSpecification, State: InformationSet<Spec>> Policy<Spec> for RandomPolicy<Spec, State>
where <<State as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::IntoIter : ExactSizeIterator{
    type StateType = State;

    fn select_action(&self, state: &Self::StateType) -> Option<Spec::ActionType> {
        let mut rng = rand::thread_rng();
        state.available_actions().into_iter().choose(&mut rng)
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>> Policy<Spec> for Box<P>{
    type StateType = P::StateType;

    fn select_action(&self, state: &Self::StateType) -> Option<Spec::ActionType> {
        self.as_ref().select_action(state)
    }
}
/*
pub trait CompatiblePolicy<Spec: ProtocolSpecification>: Policy{}
impl<Spec: ProtocolSpecification, P: Policy> CompatiblePolicy<Spec> for P
{

}

 */
//<<<StateType as InformationSet>::Id = Spec::AgentId>>{}