use crate::action::Action;
use crate::DomainEnvironment;
use crate::protocol::ProtocolSpecification;
use crate::state::env::EnvironmentState;
use crate::state::State;

pub trait StatefulEnvironment<Spec: ProtocolSpecification> : DomainEnvironment<Spec>{
    type State: EnvironmentState<Spec>;
    //type Act: Action;
    type UpdatesIterator: Iterator<Item=(Spec::AgentId, Spec::UpdateType)>;

    fn state(&self) -> &Self::State;

    fn process_action(&mut self, agent: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesIterator, Spec::GameErrorType>;

    fn current_player(&self) -> Option<Spec::AgentId>{
        self.state().current_player()
    }
}