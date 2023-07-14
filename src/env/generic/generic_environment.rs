use std::collections::{HashMap};

use log::debug;
use crate::env::{BroadcastingEnv, CommunicatingEnv, EnvironmentBuilderTrait, EnvironmentState, EnvironmentStateUniScore, EnvironmentWithAgents, ScoreEnvironment, StatefulEnvironment};
use crate::{comm::EnvCommEndpoint, Reward};
use crate::env::generic::ActionProcessor;
use crate::error::{CommError, SetupError};
use crate::protocol::{AgentMessage, DomainParameters, EnvMessage};


pub struct GenericEnv<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    AP: ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP>>{

    comm_endpoints: HashMap<DP::AgentId, C>,
    penalties: HashMap<DP::AgentId, DP::UniversalReward>,
    game_state: S,
    action_processor: AP,
}

impl <
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    PA: ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP>>
GenericEnv<DP, S, PA, C>{

    pub fn new(
        game_state: S,
        action_processor: PA,
        comm_endpoints:  HashMap<DP::AgentId, C>) -> Self{

        let k:Vec<DP::AgentId> = comm_endpoints.keys().copied().collect();
        debug!("Creating environment with:{k:?}");

        let penalties: HashMap<DP::AgentId, DP::UniversalReward> = comm_endpoints.keys()
            .map(|agent| (*agent, DP::UniversalReward::neutral()))
            .collect();

        Self{comm_endpoints, game_state, action_processor, penalties}
    }

    pub fn replace_state(&mut self, state: S){
        self.game_state = state
    }
}


impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    PA: ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP>>
StatefulEnvironment<DP> for GenericEnv<DP, S, PA, C>{

    type State = S;
    type UpdatesIterator = <Vec<(DP::AgentId, DP::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.game_state
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType) -> Result<Self::UpdatesIterator, DP::GameErrorType> {
        let updates = self.action_processor.process_action(&mut self.game_state, agent, action)?;

        Ok(updates.into_iter())

    }




}

impl<
    DP: DomainParameters,
    S: EnvironmentStateUniScore<DP>,
    AP: ActionProcessor<DP, S>,
    C: EnvCommEndpoint<DP> >
ScoreEnvironment<DP> for GenericEnv<DP, S, AP, C>{

    fn process_action_penalise_illegal(
        &mut self,
        agent: &DP::AgentId,
        action: &DP::ActionType,
        penalty_reward: DP::UniversalReward)
        -> Result<Self::UpdatesIterator, DP::GameErrorType> {

        match self.action_processor.process_action(&mut self.game_state, agent, &action){
            Ok(updates) => Ok(updates.into_iter()),
            Err(err) => {
                self.penalties.insert(*agent, penalty_reward + &self.penalties[agent]);
                Err(err)
            }
        }
    }

    fn actual_state_score_of_player(
        &self, agent: &DP::AgentId) -> DP::UniversalReward {

        self.game_state.state_score_of_player(agent)
    }

    fn actual_penalty_score_of_player
    (&self, agent: &DP::AgentId) -> DP::UniversalReward {

        self.penalties.get(agent).unwrap_or(&DP::UniversalReward::neutral()).to_owned()
    }
}



impl<Spec: DomainParameters, State: EnvironmentState<Spec>, ProcessAction: ActionProcessor<Spec, State>,Comm: EnvCommEndpoint<Spec> >
CommunicatingEnv<Spec> for GenericEnv<Spec, State, ProcessAction, Comm> {
    type CommunicationError = CommError<Spec>;

    fn send_to(&mut self, agent_id: &Spec::AgentId, message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.send(message))?
    }

    fn recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.recv())?
    }

    fn try_recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.try_recv())?
    }
}

impl <Spec: DomainParameters,
    State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec>>
BroadcastingEnv<Spec> for GenericEnv<Spec, State, ProcessAction, Comm>{
    fn send_to_all(&mut self, message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError> {
        let mut result:Option<Self::CommunicationError> = None;

        for comm in self.comm_endpoints.values_mut(){
            if let Err(sending_err) = comm.send(message.clone()){
                result = Some(sending_err)
            }
        }

        match result{
            Some(e) => Err(e),
            None => Ok(())
        }
    }
}

impl <'a, Spec: DomainParameters + 'a,
    State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec>>
 EnvironmentWithAgents<Spec> for GenericEnv<Spec, State, ProcessAction, Comm>{
    type PlayerIterator = Vec<Spec::AgentId>;

    fn players(&self) -> Self::PlayerIterator {
        self.comm_endpoints.keys().copied().collect()
    }


}


pub struct GenericEnvironmentBuilder<Spec: DomainParameters, State:EnvironmentState<Spec>,
ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec> >{
    state_opt: Option<State>,
    comm_endpoints: HashMap<Spec::AgentId,
        Comm>,

    fn_action_process: Option<ProcessAction>

}

impl <Spec: DomainParameters, State:EnvironmentState<Spec>,
ProcessAction: ActionProcessor<Spec, State> , Comm: EnvCommEndpoint<Spec>>
GenericEnvironmentBuilder<Spec, State, ProcessAction, Comm>{


    pub fn new() -> Self{
        Self{comm_endpoints: HashMap::new(), fn_action_process: None, state_opt: None}
    }


    pub fn with_processor(mut self, processor: ProcessAction) -> Result<Self, SetupError<Spec>>{
        self.fn_action_process = Some(processor);
        Ok(self)
    }



}


impl<Spec: DomainParameters, State: EnvironmentState<Spec>, PA: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec>> Default for GenericEnvironmentBuilder<Spec, State, PA, Comm> {
    fn default() -> Self {
        Self{
            state_opt: None,
            comm_endpoints: HashMap::new(),
            fn_action_process: None,
        }
    }
}

impl <Spec: DomainParameters, State:EnvironmentState<Spec>,
PA: ActionProcessor<Spec, State> , Comm: EnvCommEndpoint<Spec>>
EnvironmentBuilderTrait<Spec, GenericEnv<Spec, State, PA, Comm>> for GenericEnvironmentBuilder<Spec, State,PA, Comm >{
    type Comm = Comm;

    fn build(self) -> Result<GenericEnv<Spec, State, PA, Comm>, SetupError<Spec>>{


        Ok(GenericEnv::new(
            self.state_opt.ok_or(SetupError::MissingState)?,
            self.fn_action_process.ok_or(SetupError::<Spec>::MissingActionProcessingFunction)?,
            self.comm_endpoints))

    }

    fn add_comm(mut self, agent_id: &Spec::AgentId, comm: Comm) -> Result<Self, SetupError<Spec>>{

        let _ = &mut self.comm_endpoints.insert(*agent_id, comm);
        Ok(self)
    }

    fn with_state(mut self, state: State) -> Result<Self, SetupError<Spec>>{
        self.state_opt = Some(state);
        Ok(self)
    }
}

