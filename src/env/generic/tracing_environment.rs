use std::collections::HashMap;
use std::vec::IntoIter;
use log::debug;
use crate::comm::EnvCommEndpoint;
use crate::env::{EnvironmentState, GameHistory, HistoryEntry, StatefulEnvironment};
use crate::env::generic::{ActionProcessor, GenericEnv};
use crate::protocol::DomainParameters;
use crate::Reward;

pub struct TracingGenericEnv<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    AP:ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP>>{

    /*
    comm_endpoints: HashMap<DP::AgentId, C>,
    penalties: HashMap<DP::AgentId, DP::UniversalReward>,
    game_state: S,
    action_processor: AP,

     */
    base_environment: GenericEnv<DP, S, AP, C>,
    history: GameHistory<DP, S>
}

impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    AP:ActionProcessor<DP, S>,
    Comm: EnvCommEndpoint<DP>> TracingGenericEnv<DP, S, AP, Comm>{

    pub fn new(
        game_state: S,
        action_processor: AP,
        comm_endpoints: HashMap<DP::AgentId, Comm>) -> Self{

        /*
        let k:Vec<DP::AgentId> = comm_endpoints.keys().copied().collect();
        debug!("Creating environment with:{k:?}");

        let penalties: HashMap<DP::AgentId, DP::UniversalReward> = comm_endpoints.keys()
            .map(|agent| (*agent, DP::UniversalReward::neutral()))
            .collect();

         */

        let base_environment = GenericEnv::new(game_state, action_processor, comm_endpoints);


        Self{base_environment, history: Default::default() }
    }

    

}



impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    PA: ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP>>
StatefulEnvironment<DP> for TracingGenericEnv<DP, S, PA, C>{

    type State = S;
    type UpdatesIterator = <Vec<(DP::AgentId, DP::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.base_environment.state()
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType)
        -> Result<Self::UpdatesIterator, DP::GameErrorType> {

        let state_clone = self.state().clone();
        /*
        match self.action_processor.process_action(
            &mut self.game_state, agent, action){
            Ok(updates) => {
                self.history.push(
                    HistoryEntry::new(state_clone, *agent, action.clone(), true));
                Ok(updates.into_iter())
            }
            Err(err) => {
                self.history.push(
                    HistoryEntry::new(state_clone, *agent, action.clone(), false));
                Err(err)
            }
        }

         */
        match self.base_environment.process_action(agent, action){
            Ok(updates) => {
                self.history.push(HistoryEntry::new(state_clone, *agent, action.clone(), true));
                Ok(updates)
            }
            Err(e) => {
                self.history.push(HistoryEntry::new(state_clone, *agent, action.clone(), false));
                Err(e)
            }
        }
    }
}