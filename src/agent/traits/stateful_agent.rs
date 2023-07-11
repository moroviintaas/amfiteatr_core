use crate::agent::Agent;
use crate::protocol::DomainParameters;
use crate::state::agent::InformationSet;


pub trait StatefulAgent<DP: DomainParameters>: Agent<DP>{
    type State: InformationSet<DP>;

    fn update(&mut self, state_update: DP::UpdateType) -> Result<(), DP::GameErrorType>;
    fn state(&self) -> &Self::State;
}