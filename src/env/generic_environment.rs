use std::collections::{hash_map, HashMap};
use crate::{BroadcastingEnv, CommEndpoint, CommunicatingEnv, DomainEnvironment, EnvironmentState, EnvironmentWithAgents, GrowingEnvironment, State, StatefulEnvironment};
use crate::error::{CommError, SetupError};
use crate::error::SetupError::MissingState;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait ActionProcessingFunction<Spec: ProtocolSpecification, State: EnvironmentState<Spec>>
: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType>{

}
impl <Spec: ProtocolSpecification, State: EnvironmentState<Spec>, F> ActionProcessingFunction<Spec, State> for F
where F: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType>{

}


pub struct GenericEnvironment<Spec: ProtocolSpecification, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State> >{

    comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError<Spec>>>>,

    game_state: State,
    fn_action_process: ProcessAction,
}

impl <Spec: ProtocolSpecification, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State> > GenericEnvironment<Spec, State, ProcessAction>{


    pub fn new(game_state: State, fn_action_process: ProcessAction,
               comm_endpoints:  HashMap<Spec::AgentId,
                                     Box<dyn CommEndpoint<
                                    OutwardType=EnvMessage<Spec>,
                                    InwardType=AgentMessage<Spec>,
                                    Error=CommError<Spec>>>>
               ) -> Self{
        Self{comm_endpoints, game_state, fn_action_process}
    }
}


impl<Spec: ProtocolSpecification, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State> >
DomainEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction>{
}

impl<Spec: ProtocolSpecification, State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State> >
StatefulEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction>{
    type State = State;
    type UpdatesIterator = <Vec<(Spec::AgentId, Spec::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.game_state
    }

    fn process_action(&mut self, agent: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesIterator, Spec::GameErrorType> {
        let updates = (self.fn_action_process)(&mut self.game_state, agent, action)?;
        Ok(updates.into_iter())

    }
}

impl<Spec: ProtocolSpecification, State: EnvironmentState<Spec>, ProcessAction: ActionProcessingFunction<Spec, State> >
CommunicatingEnv<Spec> for GenericEnvironment<Spec, State, ProcessAction> {
    type CommunicationError = CommError<Spec>;

    fn send_to(&mut self, agent_id: &Spec::AgentId, message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.as_mut().send(message))?
    }

    fn recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.as_mut().recv())?
    }

    fn try_recv_from(&mut self, agent_id: &Spec::AgentId) -> Result<AgentMessage<Spec>, Self::CommunicationError> {
        self.comm_endpoints.get_mut(agent_id).ok_or(CommError::NoSuchConnection)
            .map(|v| v.as_mut().try_recv())?
    }
}

impl <Spec: ProtocolSpecification,
    State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State>>
BroadcastingEnv<Spec> for GenericEnvironment<Spec, State, ProcessAction>{
    fn send_to_all(&mut self, message: EnvMessage<Spec>) -> Result<(), Self::CommunicationError> {
        let mut result:Option<Self::CommunicationError> = None;

        for comm in self.comm_endpoints.values_mut(){
            if let Err(sending_err) = comm.as_mut().send(message.clone()){
                result = Some(sending_err)
            }
        }

        match result{
            Some(e) => Err(e),
            None => Ok(())
        }
    }
}

impl <'a, Spec: ProtocolSpecification + 'a,
    State: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, State>>
 EnvironmentWithAgents<Spec> for GenericEnvironment<Spec, State, ProcessAction>{
    type PlayerIterator = Vec<Spec::AgentId>;

    fn players(&self) -> Self::PlayerIterator {
        self.comm_endpoints.keys().into_iter().map(|k| *k).collect()
    }
}
/*
impl <Spec: ProtocolSpecification, State: EnvironmentState<Spec>,
    ProcessAction: Fn(&mut State, &Spec::AgentId, Spec::ActionType)
        -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType>>
GrowingEnvironment<Spec> for GenericEnvironment<Spec, State, ProcessAction>{
    type Endpoint = Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError>>;

    fn add_connection(&mut self, agent_id: Spec::AgentId, endpoint: Self::Endpoint) -> Result<(), SetupError<Spec>> {
        todo!()
    }
}*/

//#[derive(Default)]
pub struct GenericEnvironmentBuilder<Spec: ProtocolSpecification, State:EnvironmentState<Spec>,
ProcessAction: ActionProcessingFunction<Spec, State> >{
    state_opt: Option<State>,
    comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError<Spec>>>>,

    fn_action_process: Option<ProcessAction>

}

impl <Spec: ProtocolSpecification, State:EnvironmentState<Spec>,
ProcessAction: ActionProcessingFunction<Spec, State>  >
GenericEnvironmentBuilder<Spec, State, ProcessAction>{

    /*pub fn init_builder(state: State, fn_action_process: ProcessAction) -> Self{
        Self{state_opt: Some(state), fn_action_process: Some(fn_action_process), comm_endpoints: HashMap::new()}
    }*/

    pub fn new() -> Self{
        Self{comm_endpoints: HashMap::new(), fn_action_process: None, state_opt: None}
    }

    pub fn with_state(mut self, state: State) -> Result<Self, SetupError<Spec>>{
        self.state_opt = Some(state);
        Ok(self)
    }
    pub fn with_processor(mut self, processor: ProcessAction) -> Result<Self, SetupError<Spec>>{
        self.fn_action_process = Some(processor);
        Ok(self)
    }
    pub fn add_comm(mut self, agent_id: &Spec::AgentId, comm: Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError<Spec>>>) -> Result<Self, SetupError<Spec>>{

        //let mut hm = std::mem::take(&mut self.comm_endpoints);
        &mut self.comm_endpoints.insert(*agent_id, comm);
        Ok(self)
        //self.comm_endpoints.insert(agent_id, comm)
    }

    pub fn build(self) -> Result<GenericEnvironment<Spec, State, ProcessAction>, SetupError<Spec>>{


        Ok(GenericEnvironment::new(
            self.state_opt.ok_or(SetupError::MissingState)?,
            self.fn_action_process.ok_or(SetupError::<Spec>::MissingActionProcessingFunction)?,
            self.comm_endpoints))

    }
}