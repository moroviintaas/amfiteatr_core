use crate::agent::Agent;
use crate::agent::info_set::InformationSet;
use crate::domain::DomainParameters;


pub trait StatefulAgent<DP: DomainParameters>: Agent<DP>{
    type State: InformationSet<DP>;

    fn update(&mut self, state_update: DP::UpdateType) -> Result<(), DP::GameErrorType>;
    fn state(&self) -> &Self::State;
}