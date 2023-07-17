
use crate::protocol::DomainParameters;

pub trait EnvironmentState<DP: DomainParameters>: Clone{
    type Updates: IntoIterator<Item = (DP::AgentId, DP::UpdateType)>;

    fn current_player(&self) -> Option<DP::AgentId>;
    fn is_finished(&self) -> bool;

    fn forward(&mut self, agent: DP::AgentId, action: DP::ActionType)
        -> Result<Self::Updates, DP::GameErrorType>;

    //fn transform(&mut self, agent_id: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesCollection, Spec::GameErrorType>;

}


