/*use std::collections::HashMap;
use std::marker::PhantomData;
use ron::Error;
use serde::{Deserialize, Serialize};
use crate::automatons::rr::{AgentRR, EnvironmentRR};
use crate::{AgentGen, Environment, InformationSet, Policy, SyncComm};
use crate::error::CommError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
*/
/*
#[derive(Debug, Clone)]
pub struct LocalSymmetricSimulation<
    Spec: ProtocolSpecification,
    Env: EnvironmentRR<Spec>,
    Ag: AgentRR<Spec>> {

    _spec: PhantomData<Spec>,
    environment: Env,
    agents: HashMap<Spec::AgentId, Ag>
}

impl<
    Spec: ProtocolSpecification,
    Env: EnvironmentRR<Spec>,
    Agent: AgentRR<Spec>> LocalSymmetricSimulation<Spec, Env, Agent>{

    pub fn new(environment: Env, agents: HashMap<Spec::AgentId, Ag>) -> Self{
        Self{environment, agents, _spec: PhantomData::default()}
    }
}
 */
/*
pub struct LocalSimulation<
    Spec: ProtocolSpecification,
    Env: EnvironmentRR<Spec>>{

    environment: Env,
    _spec: PhantomData<Spec>,
    agents: HashMap<Spec::AgentId, AgentGen<Spec, Box<dyn Policy<StateType=
    Box<dyn InformationSet<ActionType = Spec::ActionType, Er>>>>, SyncComm<AgentMessage<Spec>, EnvMessage<Spec>, CommError>>>
}
*/
