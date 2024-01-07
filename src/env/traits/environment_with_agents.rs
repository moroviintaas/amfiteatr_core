
use crate::domain::DomainParameters;
/*
pub trait EnvironmentWithAgents<'a, Id: AgentIdentifier + 'a>{
    type PlayerIterator: IntoIterator<Item = &'a Id>;

    fn players(&self) -> Self::PlayerIterator;
}*/


/// Environment interface to list agents taking part in game or simulations
///
pub trait EnvironmentWithAgents<Spec: DomainParameters>{
    type PlayerIterator: IntoIterator<Item = Spec::AgentId>;

    /// Method returning `IntoIterator` of players in game.
    /// This method is used to propagate error so it should include all players.
    fn players(&self) -> Self::PlayerIterator;


}