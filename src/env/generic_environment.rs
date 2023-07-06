use std::collections::{HashMap};
use std::hash::Hash;

use log::debug;
use crate::{BroadcastingEnv, CommunicatingEnv, DomainEnvironment, EnvironmentState, EnvironmentWithAgents, StatefulEnvironment, EnvCommEndpoint, EnvironmentBuilderTrait, EnvironmentStateUniScore, ScoreEnvironment, Reward};
use crate::error::{CommError, SetupError};

use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};
#[allow(clippy::type_complexity)]
pub trait ActionProcessor<Spec: DomainParameters, State: EnvironmentState<Spec>> {

    fn process_action(
        &self,
        state: &mut State,
        agent_id: &Spec::AgentId,
        action: Spec::ActionType)
        -> Result<Vec<(Spec::AgentId, Spec::UpdateType)>, Spec::GameErrorType>;


}



/*
impl <Spec: ProtocolSpecification, State: EnvironmentState<Spec>, F> ActionProcessor<Spec, State> for F
where F: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType>{

}

p
*/

pub struct GenericEnvironment<Spec: DomainParameters, State: EnvironmentState<Spec>,
    AP: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec>>{

    comm_endpoints: HashMap<Spec::AgentId, Comm>,
    penalties: HashMap<Spec::AgentId, Spec::UniversalReward>,
    //penalties: HashMap<Spec::AgentId, Spec::UniversalReward>,

    game_state: State,
    action_processor: AP,
}

impl <Spec: DomainParameters, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec>> GenericEnvironment<Spec, State, ProcessAction, Comm>{


    pub fn new(game_state: State, action_processor: ProcessAction,
               comm_endpoints:  HashMap<Spec::AgentId,  Comm>
               ) -> Self{
        /*let penalties: HashMap<Spec::AgentId, Spec::UniversalReward> = comm_endpoints.iter()
            .map(|(id, _)|{
                (id.clone(), Spec::UniversalReward::neutral())
            }).collect();*/
        let k:Vec<Spec::AgentId> = comm_endpoints.keys().copied().collect();
        debug!("Creating environment with:{k:?}");

        let penalties: HashMap<Spec::AgentId, Spec::UniversalReward> = comm_endpoints.iter()
            .map(|(agent, _)| (agent.clone(), Spec::UniversalReward::neutral()))
            .collect();

        Self{comm_endpoints, game_state, action_processor, penalties}
    }

    pub fn replace_state(&mut self, state: State){
        self.game_state = state
    }
}


impl<Spec: DomainParameters, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec> >
DomainEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm>{
}

impl<Spec: DomainParameters, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec> >
StatefulEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm>{
    type State = State;
    type UpdatesIterator = <Vec<(Spec::AgentId, Spec::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.game_state
    }

    fn process_action(&mut self, agent: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesIterator, Spec::GameErrorType> {
        let updates = self.action_processor.process_action(&mut self.game_state, agent, action)?;
        //let (self.state, updates) = self.action_processor(self.game_state)

        Ok(updates.into_iter())

    }




}

impl<Spec: DomainParameters, State: EnvironmentStateUniScore<Spec>,
    ProcessAction: ActionProcessor<Spec, State>, Comm: EnvCommEndpoint<Spec> >
ScoreEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm>{
    fn process_action_penalise_illegal(&mut self, agent: &Spec::AgentId, action: Spec::ActionType, penalty_reward: Spec::UniversalReward) -> Result<Self::UpdatesIterator, Spec::GameErrorType> {
        match self.action_processor.process_action(&mut self.game_state, agent, action){
            Ok(updates) => Ok(updates.into_iter()),
            Err(err) => {
                self.penalties.insert(*agent, penalty_reward + &self.penalties[agent]);
                Err(err)
            }
        }
        //let updates = self.action_processor.process_action_penalise_illegal(&mut self.game_state, agent, action, penalty_reward)?;

    }

    fn actual_state_score_of_player(&self, agent: &Spec::AgentId) -> Spec::UniversalReward {
        self.game_state.state_score_of_player(agent)
    }

    fn actual_penalty_score_of_player(&self, agent: &Spec::AgentId) -> Spec::UniversalReward {
        self.penalties[agent].clone()
    }
}



impl<Spec: DomainParameters, State: EnvironmentState<Spec>, ProcessAction: ActionProcessor<Spec, State>,Comm: EnvCommEndpoint<Spec> >
CommunicatingEnv<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm> {
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
BroadcastingEnv<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm>{
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
 EnvironmentWithAgents<Spec> for GenericEnvironment<Spec, State, ProcessAction, Comm>{
    type PlayerIterator = Vec<Spec::AgentId>;

    fn players(&self) -> Self::PlayerIterator {
        self.comm_endpoints.keys().copied().collect()
    }


}


//#[derive(Default)]
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

    /*pub fn init_builder(state: State, fn_action_process: ProcessAction) -> Self{
        Self{state_opt: Some(state), fn_action_process: Some(fn_action_process), comm_endpoints: HashMap::new()}
    }*/

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
EnvironmentBuilderTrait<Spec, GenericEnvironment<Spec, State, PA, Comm>> for GenericEnvironmentBuilder<Spec, State,PA, Comm >{
    type Comm = Comm;

    fn build(self) -> Result<GenericEnvironment<Spec, State, PA, Comm>, SetupError<Spec>>{


        Ok(GenericEnvironment::new(
            self.state_opt.ok_or(SetupError::MissingState)?,
            self.fn_action_process.ok_or(SetupError::<Spec>::MissingActionProcessingFunction)?,
            self.comm_endpoints))

    }

    fn add_comm(mut self, agent_id: &Spec::AgentId, comm: Comm) -> Result<Self, SetupError<Spec>>{

        //let mut hm = std::mem::take(&mut self.comm_endpoints);
        let _ = &mut self.comm_endpoints.insert(*agent_id, comm);
        Ok(self)
        //self.comm_endpoints.insert(agent_id, comm)
    }

    fn with_state(mut self, state: State) -> Result<Self, SetupError<Spec>>{
        self.state_opt = Some(state);
        Ok(self)
    }
}