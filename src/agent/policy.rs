use std::marker::PhantomData;
use crate::state::agent::InformationSet;
use rand::seq::IteratorRandom;
use crate::protocol::DomainParameters;


pub trait Policy<Spec: DomainParameters>{
    type StateType: InformationSet<Spec>;

    fn select_action(&self, state: &Self::StateType) -> Option<Spec::ActionType>;
    fn select_action_mut(&mut self, state: &Self::StateType) -> Option<Spec::ActionType>{
        self.select_action(state)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RandomPolicy<Spec: DomainParameters, State: InformationSet<Spec>>{
    state: PhantomData<State>,
    _spec: PhantomData<Spec>
}
impl<Spec: DomainParameters, State: InformationSet<Spec>> RandomPolicy<Spec, State>{
    pub fn new() -> Self{
        Self{state: PhantomData::default(), _spec: PhantomData::default()}
    }
}

impl<Spec: DomainParameters, State: InformationSet<Spec>> Policy<Spec> for RandomPolicy<Spec, State>
where <<State as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::IntoIter : ExactSizeIterator{
    type StateType = State;

    fn select_action(&self, state: &Self::StateType) -> Option<Spec::ActionType> {
        let mut rng = rand::thread_rng();
        state.available_actions().into_iter().choose(&mut rng)
    }
}

impl<Spec: DomainParameters, P: Policy<Spec>> Policy<Spec> for Box<P>{
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