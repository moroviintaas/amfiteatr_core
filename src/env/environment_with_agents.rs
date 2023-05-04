
use crate::protocol::ProtocolSpecification;
/*
pub trait EnvironmentWithAgents<'a, Id: AgentIdentifier + 'a>{
    type PlayerIterator: IntoIterator<Item = &'a Id>;

    fn players(&self) -> Self::PlayerIterator;
}*/

pub trait EnvironmentWithAgents<Spec: ProtocolSpecification>{
    type PlayerIterator: IntoIterator<Item = Spec::AgentId>;

    fn players(&self) -> Self::PlayerIterator;


}