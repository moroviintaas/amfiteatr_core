use crate::agent::AgentIdentifier;
use crate::state::State;

pub trait EnvironmentState: State{
    type AgentId: AgentIdentifier;

    fn current_player(&self) -> Option<Self::AgentId>;
}

