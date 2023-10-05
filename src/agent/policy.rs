use std::marker::PhantomData;
use crate::state::agent::InformationSet;
use rand::seq::IteratorRandom;
use crate::domain::DomainParameters;


pub trait Policy<DP: DomainParameters>: Send{
    type InfoSetType: InformationSet<DP>;

    fn select_action(&self, state: &Self::InfoSetType) -> Option<DP::ActionType>;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RandomPolicy<DP: DomainParameters, State: InformationSet<DP>>{
    state: PhantomData<State>,
    _spec: PhantomData<DP>
}
impl<DP: DomainParameters, InfoSet: InformationSet<DP>> RandomPolicy<DP, InfoSet>{
    pub fn new() -> Self{
        Self{state: PhantomData::default(), _spec: PhantomData::default()}
    }
}

impl<DP: DomainParameters, InfoSet: InformationSet<DP>> Policy<DP> for RandomPolicy<DP, InfoSet>
where <<InfoSet as InformationSet<DP>>::ActionIteratorType as IntoIterator>::IntoIter : ExactSizeIterator{
    type InfoSetType = InfoSet;

    fn select_action(&self, state: &Self::InfoSetType) -> Option<DP::ActionType> {
        let mut rng = rand::thread_rng();
        state.available_actions().into_iter().choose(&mut rng)
    }
}

impl<DP: DomainParameters, P: Policy<DP>> Policy<DP> for Box<P>{
    type InfoSetType = P::InfoSetType;

    fn select_action(&self, state: &Self::InfoSetType) -> Option<DP::ActionType> {
        self.as_ref().select_action(state)
    }
}
