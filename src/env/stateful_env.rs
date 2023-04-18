use crate::action::Action;
use crate::DomainEnvironment;
use crate::protocol::ProtocolSpecification;
use crate::state::env::EnvironmentState;
use crate::state::State;

pub trait StatefulEnvironment : DomainEnvironment{
    type State: EnvironmentState<AgentId = <Self::DomainParameter as ProtocolSpecification>::AgentId,
        UpdateType = <<Self as DomainEnvironment>::DomainParameter as ProtocolSpecification>::UpdateType>;
    type Act: Action;
    type UpdatesIterator: Iterator<Item=(<Self::State as EnvironmentState>::AgentId, <Self::State as State>::UpdateType)>;

    fn state(&self) -> &Self::State;

    fn process_action(&mut self, agent: &<Self::State as EnvironmentState>::AgentId, action: Self::Act) -> Result<Self::UpdatesIterator, <Self::State as State>::Error>;

    fn current_player(&self) -> Option<<Self::State as EnvironmentState>::AgentId>{
        self.state().current_player()
    }
}