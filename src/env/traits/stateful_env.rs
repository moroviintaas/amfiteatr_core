use crate::domain::DomainParameters;
use crate::env::EnvStateSequential;


pub trait StatefulEnvironment<DP: DomainParameters>{
    type State: EnvStateSequential<DP>;

    fn state(&self) -> &Self::State;



    fn current_player(&self) -> Option<DP::AgentId>{
        self.state().current_player()
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType) 
        -> Result<<Self::State as EnvStateSequential<DP>>::Updates, DP::GameErrorType>;

}