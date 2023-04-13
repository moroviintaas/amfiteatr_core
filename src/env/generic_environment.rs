use std::collections::HashMap;
use crate::{CommEndpoint, EnvironmentState, State, StatefulEnvironment};
use crate::error::CommError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
/*
pub struct GenericEnvironment<Spec: ProtocolSpecification, E: EnvironmentState>{
    comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<S>,
            InwardType=AgentMessage<S>,
            Error=CommError>>>,

    game_state: E
}

*/