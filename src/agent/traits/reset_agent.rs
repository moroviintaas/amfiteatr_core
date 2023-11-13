use crate::agent::StatefulAgent;
use crate::domain::DomainParameters;

/// Trait for agent that can reset their attributes to some default values
/// while setting new info set. Typically to be used in situations
/// when game is to be relaunched from beginning (optionally with new start point)
pub trait ReinitAgent<DP: DomainParameters>: StatefulAgent<DP>{

    fn reinit(&mut self, new_info_set: <Self as StatefulAgent<DP>>::InfoSetType);
}