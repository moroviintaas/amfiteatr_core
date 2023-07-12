use crate::agent::StatefulAgent;
use crate::protocol::DomainParameters;

pub trait ResetAgent<DP: DomainParameters>: StatefulAgent<DP>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::State);
}