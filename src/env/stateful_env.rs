
use crate::DomainEnvironment;
use crate::protocol::ProtocolSpecification;
use crate::state::env::EnvironmentState;


pub trait StatefulEnvironment<Spec: ProtocolSpecification> : DomainEnvironment<Spec>{
    type State: EnvironmentState<Spec>;
    //type Act: Action;
    type UpdatesIterator: Iterator<Item=(Spec::AgentId, Spec::UpdateType)>;

    fn state(&self) -> &Self::State;



    fn current_player(&self) -> Option<Spec::AgentId>{
        self.state().current_player()
    }

    fn process_action(&mut self, agent: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesIterator, Spec::GameErrorType>;
}