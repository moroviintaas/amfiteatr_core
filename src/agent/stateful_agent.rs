use crate::protocol::ProtocolSpecification;
use crate::state::agent::InformationSet;


pub trait StatefulAgent<Spec: ProtocolSpecification>{
    type State: InformationSet<Spec>;

    fn update(&mut self, state_update: Spec::UpdateType) -> Result<(), Spec::GameErrorType>;
    fn state(&self) -> &Self::State;
}