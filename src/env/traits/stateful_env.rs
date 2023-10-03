use crate::domain::DomainParameters;
use crate::state::env::EnvironmentState;


pub trait StatefulEnvironment<DP: DomainParameters>{
    type State: EnvironmentState<DP>;

    fn state(&self) -> &Self::State;



    fn current_player(&self) -> Option<DP::AgentId>{
        self.state().current_player()
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType) 
        -> Result<<Self::State as EnvironmentState<DP>>::Updates, DP::GameErrorType>;

}