use std::collections::{HashMap};

use log::debug;
use crate::env::{BroadcastingEnv, CommunicatingEnv, EnvironmentBuilderTrait, EnvironmentState, EnvironmentStateUniScore, EnvironmentWithAgents, ResetEnvironment, ScoreEnvironment, StatefulEnvironment};
use crate::{comm::EnvCommEndpoint};
use crate::error::{CommError, SetupError};
use crate::domain::{AgentMessage, DomainParameters, EnvMessage, Reward};


pub struct HashMapEnv<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>{

    comm_endpoints: HashMap<DP::AgentId, C>,
    penalties: HashMap<DP::AgentId, DP::UniversalReward>,
    game_state: S,
}

impl <
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
HashMapEnv<DP, S, C>{

    pub fn new(
        game_state: S,
        comm_endpoints:  HashMap<DP::AgentId, C>) -> Self{

        let k:Vec<DP::AgentId> = comm_endpoints.keys().copied().collect();
        debug!("Creating environment with:{k:?}");

        let penalties: HashMap<DP::AgentId, DP::UniversalReward> = comm_endpoints.keys()
            .map(|agent| (*agent, DP::UniversalReward::neutral()))
            .collect();

        Self{comm_endpoints, game_state, penalties}
    }

    pub fn replace_state(&mut self, state: S){
        self.game_state = state
    }
}


impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
StatefulEnvironment<DP> for HashMapEnv<DP, S, C>{

    type State = S;
    //type Updates = <Vec<(DP::AgentId, DP::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.game_state
    }

    fn process_action(&mut self, agent: &DP::AgentId, action: &DP::ActionType) 
        -> Result<<Self::State as EnvironmentState<DP>>::Updates, DP::GameErrorType> {
        //let updates = self.action_processor.process_action(&mut self.game_state, agent, action)?;
        self.game_state.forward(*agent, action.clone())
        //Ok(updates)

    }




}

impl<
    DP: DomainParameters,
    S: EnvironmentStateUniScore<DP>,
    C: EnvCommEndpoint<DP> >
ScoreEnvironment<DP> for HashMapEnv<DP, S, C>{

    fn process_action_penalise_illegal(
        &mut self,
        agent: &DP::AgentId,
        action: &DP::ActionType,
        penalty_reward: DP::UniversalReward)
        -> Result<<Self::State as EnvironmentState<DP>>::Updates, DP::GameErrorType> {


        self.game_state.forward(*agent, action.clone()).map_err(|e|{
            self.penalties.insert(*agent, penalty_reward + &self.penalties[agent]);
            e
        })
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



impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
CommunicatingEnv<DP> for HashMapEnv<DP, S, C> {
    type CommunicationError = CommError<DP>;

    fn send_to(&mut self, agent_id: &DP::AgentId, message: EnvMessage<DP>)
        -> Result<(), Self::CommunicationError> {

        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.send(message))?
    }

    fn recv_from(&mut self, agent_id: &DP::AgentId)
        -> Result<AgentMessage<DP>, Self::CommunicationError> {

        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.recv())?
    }

    fn try_recv_from(&mut self, agent_id: &DP::AgentId)
        -> Result<AgentMessage<DP>, Self::CommunicationError> {

        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.try_recv())?
    }
}

impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
BroadcastingEnv<DP> for HashMapEnv<DP, S, C>{
    fn send_to_all(&mut self, message: EnvMessage<DP>) -> Result<(), Self::CommunicationError> {
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

impl<'a, DP: DomainParameters + 'a,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
 EnvironmentWithAgents<DP> for HashMapEnv<DP, S, C>{
    type PlayerIterator = Vec<DP::AgentId>;

    fn players(&self) -> Self::PlayerIterator {
        self.comm_endpoints.keys().copied().collect()
    }


}


pub struct GenericEnvironmentBuilder<
    DP: DomainParameters,
    S:EnvironmentState<DP>,
    C: EnvCommEndpoint<DP> >{
    state_opt: Option<S>,
    comm_endpoints: HashMap<DP::AgentId,  C>,


}

impl <DP: DomainParameters, S:EnvironmentState<DP>, C: EnvCommEndpoint<DP>>
GenericEnvironmentBuilder<DP, S, C>{


    pub fn new() -> Self{
        Self{comm_endpoints: HashMap::new(),  state_opt: None}
    }


}


impl<
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
Default for GenericEnvironmentBuilder<DP, S, C> {

    fn default() -> Self {
        Self{
            state_opt: None,
            comm_endpoints: HashMap::new(),
        }
    }
}

impl<
    DP: DomainParameters,
    S:EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
EnvironmentBuilderTrait<DP, HashMapEnv<DP, S, C>> for GenericEnvironmentBuilder<DP, S, C>{
    type Comm = C;

    fn build(self) -> Result<HashMapEnv<DP, S, C>, SetupError<DP>>{


        Ok(HashMapEnv::new(
            self.state_opt.ok_or(SetupError::MissingState)?,
            self.comm_endpoints))

    }

    fn add_comm(mut self, agent_id: &DP::AgentId, comm: C) -> Result<Self, SetupError<DP>>{

        let _ = &mut self.comm_endpoints.insert(*agent_id, comm);
        Ok(self)
    }

    fn with_state(mut self, state: S) -> Result<Self, SetupError<DP>>{
        self.state_opt = Some(state);
        Ok(self)
    }
}

impl<
DP: DomainParameters,
    S:EnvironmentState<DP>,
    C: EnvCommEndpoint<DP>>
ResetEnvironment<DP> for HashMapEnv<DP, S, C>{
    fn reset(&mut self, initial_state: <Self as StatefulEnvironment<DP>>::State) {
        self.game_state = initial_state;
        for vals in self.penalties.values_mut(){
            *vals = DP::UniversalReward::neutral();
        }
    }
}
