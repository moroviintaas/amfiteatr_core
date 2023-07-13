use crate::env::EnvironmentState;
use crate::protocol::{DomainParameters};
#[allow(clippy::type_complexity)]
pub trait ActionProcessor<Spec: DomainParameters, State: EnvironmentState<Spec>> {

    fn process_action(
        &self,
        state: &mut State,
        agent_id: &Spec::AgentId,
        action: &Spec::ActionType)
        -> Result<Vec<(Spec::AgentId, Spec::UpdateType)>, Spec::GameErrorType>;
}

