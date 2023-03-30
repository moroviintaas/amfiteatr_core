use crate::action::Action;
use crate::state::env::EnvironmentState;
use crate::state::State;

pub trait StatefulEnvironment{
    type State: EnvironmentState;
    type Act: Action;
    type UpdatesIterator: Iterator<Item=(<Self::State as EnvironmentState>::AgentId, <Self::State as State>::UpdateType)>;

    fn state(&self) -> &Self::State;

    fn process_action(&mut self, agent: &<Self::State as EnvironmentState>::AgentId, action: Self::Act) -> Result<Self::UpdatesIterator, <Self::State as State>::Error>;

    fn current_player(&self) -> Option<<Self::State as EnvironmentState>::AgentId>{
        self.state().current_player()
    }
}