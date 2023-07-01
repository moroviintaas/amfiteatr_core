use crate::protocol::DomainParameters;
use crate::state::agent::InformationSet;


pub trait StatefulAgent<Spec: DomainParameters>{
    type State: InformationSet<Spec>;

    fn update(&mut self, state_update: Spec::UpdateType) -> Result<(), Spec::GameErrorType>;
    fn state(&self) -> &Self::State;
}