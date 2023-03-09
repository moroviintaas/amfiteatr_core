use crate::agent::AgentIdentifier;

pub trait Environment<'a, Id: AgentIdentifier + 'a>{
    type PlayerIterator: IntoIterator<Item = &'a Id>;

    fn players(&self) -> Self::PlayerIterator;
}