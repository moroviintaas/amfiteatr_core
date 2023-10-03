use crate::env::StatefulEnvironment;
use crate::domain::DomainParameters;

pub trait ResetEnvironment<DP: DomainParameters>: StatefulEnvironment<DP>{
    fn reset(&mut self, initial_state: <Self as StatefulEnvironment<DP>>::State);

}