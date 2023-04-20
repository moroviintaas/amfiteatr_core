use std::collections::HashMap;
use std::fmt::Error;
use crate::{CommEndpoint, CommunicatingEnv, DomainEnvironment, EnvironmentState, GenericEnvironment, StatefulEnvironment};
use crate::automatons::rr::EnvironmentRR;
use crate::error::{CommError, SetupError};
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub trait EnvironmentBuilderTrait<Spec: ProtocolSpecification>: Default{


    type Environment: CommunicatingEnv<Spec,  CommunicationError = CommError> + StatefulEnvironment<Spec>;
    //type Environment: EnvironmentRR<Spec = Self::ProtocolSpec>;
    type Comm: CommEndpoint;

    fn build(self) -> Self::Environment;
    fn add_comm(self, agent_id: &Spec::AgentId, comm: Self::Comm) -> Result<Self, SetupError<Spec>>;
    fn with_state(self, state: <Self::Environment as StatefulEnvironment<Spec>>::State) -> Result<Self, SetupError<Spec>>;

}

pub struct EnvironmentBuilder<Spec: ProtocolSpecification, State:EnvironmentState<Spec>,
ProcessAction: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType> >{
    state_opt: Option<State>,
    comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError>>>,

    fn_action_process: Option<ProcessAction>

}

impl <Spec: ProtocolSpecification, State:EnvironmentState<Spec>,
ProcessAction: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), Spec::GameErrorType> >
EnvironmentBuilder<Spec, State, ProcessAction>{

    pub fn init_builder(state: State, fn_action_process: ProcessAction) -> Self{
        Self{state_opt: Some(state), fn_action_process: Some(fn_action_process), comm_endpoints: HashMap::new()}
    }

    pub fn add_comm(mut self, agent_id: &Spec::AgentId, comm: Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError>>) -> Result<Self, SetupError<Spec>>{

        //let mut hm = std::mem::take(&mut self.comm_endpoints);
        &mut self.comm_endpoints.insert(*agent_id, comm);
        Ok(self)
        //self.comm_endpoints.insert(agent_id, comm)
    }

    pub fn build(self) -> Result<GenericEnvironment<Spec, State, ProcessAction>, SetupError<Spec>>{
        todo!()
    }
}