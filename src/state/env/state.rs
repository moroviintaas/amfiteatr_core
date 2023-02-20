use crate::agent::AgentIdentifier;
use crate::state::State;

pub trait EnvironmentState: State{
    type PlayerId: AgentIdentifier;

    fn current_player(&self) -> Option<Self::PlayerId>;
}