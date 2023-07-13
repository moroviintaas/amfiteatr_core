use crate::protocol::DomainParameters;
use crate::state::env::EnvironmentState;


pub trait StatefulEnvironment<DP: DomainParameters>{
    type State: EnvironmentState<DP>;
    //type Act: Action;
    type UpdatesIterator: Iterator<Item=(DP::AgentId, DP::UpdateType)>;

    fn state(&self) -> &Self::State;



    fn current_player(&self) -> Option<DP::AgentId>{
        self.state().current_player()
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType) -> Result<Self::UpdatesIterator, DP::GameErrorType>;

}