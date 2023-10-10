use crate::agent::Agent;
use crate::agent::info_set::InformationSet;
use crate::domain::DomainParameters;

/// Agent that holds some game state
/// > Formally agent knows _information set_ which can be described as state of the game
/// from point of view of the agent.
pub trait StatefulAgent<DP: DomainParameters>: Agent<DP>{
    type InfoSetType: InformationSet<DP>;

    /// Updated underlying information set using domain's updated type.
    fn update(&mut self, info_set_update: DP::UpdateType) -> Result<(), DP::GameErrorType>;
    /// Return reference to underlying information set.
    fn info_set(&self) -> &Self::InfoSetType;
}