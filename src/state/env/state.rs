
use crate::protocol::DomainParameters;
use crate::state::State;

pub trait EnvironmentState<Spec: DomainParameters>: State<Spec>{
    //type UpdatesCollection: IntoIterator<Item = (Spec::AgentId, Spec::UpdateType)>;
    //type AgentId: AgentIdentifier;

    fn current_player(&self) -> Option<Spec::AgentId>;
    //fn transform(&mut self, agent_id: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesCollection, Spec::GameErrorType>;
}

