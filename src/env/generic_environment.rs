use std::collections::HashMap;
use crate::{CommEndpoint, DomainEnvironment, EnvironmentState, State, StatefulEnvironment};
use crate::error::CommError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub struct GenericEnvironment<Spec: ProtocolSpecification, State: EnvironmentState<UpdateType = Spec::UpdateType, AgentId=Spec::AgentId>,
    ProcessAction: Fn(&mut State, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), State::Error> >{

    comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError>>>,

    game_state: State,
    fn_action_process: ProcessAction,
}

impl <Spec: ProtocolSpecification, E: EnvironmentState<UpdateType = Spec::UpdateType, AgentId=Spec::AgentId>,
    ProcessAction: Fn(&mut E, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), E::Error>> GenericEnvironment<Spec, E, ProcessAction>{


    pub fn new(comm_endpoints:  HashMap<Spec::AgentId,
                                     Box<dyn CommEndpoint<
                                    OutwardType=EnvMessage<Spec>,
                                    InwardType=AgentMessage<Spec>,
                                    Error=CommError>>>,
                game_state: E, fn_action_process: ProcessAction) -> Self{
        Self{comm_endpoints, game_state, fn_action_process}
    }
}


impl<Spec: ProtocolSpecification, E: EnvironmentState<UpdateType = Spec::UpdateType, AgentId=Spec::AgentId>,
    ProcessAction: Fn(&mut E, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), E::Error>>
DomainEnvironment for GenericEnvironment<Spec, E, ProcessAction>{
    type DomainParameter = Spec;
}

impl<Spec: ProtocolSpecification, E: EnvironmentState<UpdateType = Spec::UpdateType, AgentId=Spec::AgentId>,
    ProcessAction: Fn(&mut E, &Spec::AgentId, Spec::ActionType) -> Result<(Vec<(Spec::AgentId, Spec::UpdateType)>), E::Error>>
StatefulEnvironment for GenericEnvironment<Spec, E, ProcessAction>{
    type State = E;
    type Act = Spec::ActionType;
    type UpdatesIterator = <Vec<(Spec::AgentId, Spec::UpdateType)> as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.game_state
    }

    fn process_action(&mut self, agent: &<Self::State as EnvironmentState>::AgentId, action: Self::Act) -> Result<Self::UpdatesIterator, <Self::State as State>::Error> {
        let updates = (self.fn_action_process)(&mut self.game_state, agent, action)?;
        Ok(updates.into_iter())

    }
}
