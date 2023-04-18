use crate::agent::AgentIdentifier;
use crate::protocol::ProtocolSpecification;
use crate::state::State;

pub trait EnvironmentState<Spec: ProtocolSpecification>: State<Spec>{
    //type AgentId: AgentIdentifier;

    fn current_player(&self) -> Option<Spec::AgentId>;
}

