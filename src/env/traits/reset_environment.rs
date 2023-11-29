use crate::env::StatefulEnvironment;
use crate::domain::DomainParameters;

pub trait ReinitEnvironment<DP: DomainParameters>: StatefulEnvironment<DP>{
    fn reinit(&mut self, initial_state: <Self as StatefulEnvironment<DP>>::State);

}

pub trait ReseedEnvironment<DP: DomainParameters, Seed>: StatefulEnvironment<DP>
{
    fn reseed(&mut self, seed: Seed);
}