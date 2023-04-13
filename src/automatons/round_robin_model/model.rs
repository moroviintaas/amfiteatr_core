use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Index;
use crate::automatons::rr::{AgentRR, EnvironmentRR};
use crate::{ActingAgent, AgentGen, CommEndpoint, EnvironmentBuilder, EnvironmentState, InformationSet, Policy, Reward, StatefulAgent, StatefulEnvironment, SyncComm, SyncCommEnv};
use crate::error::{CommError, SetupError, SztormError};
use crate::error::SetupError::DuplicateId;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};

pub struct RoundRobinModel<S: ProtocolSpecification, B: EnvironmentBuilder<ProtocolSpec=S>>{
    builder: B,
    //_spec: PhantomData<S>,

    //environment_state: E,
    local_agents: HashMap<S::AgentId, Box<dyn AgentRR<S> + Send>>,
    comm_endpoints: HashMap<S::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<S>,
            InwardType=AgentMessage<S>,
            Error=CommError>>
        >




}

impl<S: ProtocolSpecification, B: EnvironmentBuilder<ProtocolSpec=S>> RoundRobinModel<S, B>{
    pub fn with_env_state(&mut self, environment_state: <B::Environment as StatefulEnvironment>::State){
        self.builder.with_state(environment_state)
    }
    pub fn get_agent(&self, s: &S::AgentId) -> Option<&Box<dyn AgentRR<S> + Send>>{
        self.local_agents.get(s).and_then(|a| Some(a))

    }


    /*pub fn add_local_agent(&mut self,
                           agent_id: &S::AgentId,
                           information_set: Box<dyn InformationSet<
                               ActionIteratorType=Vec<S::ActionType>,
                               ActionType=S::ActionType,
                               Error: Into<SztormError<S>>,
                               Id=S::AgentId,
                               RewardType: Reward,
                               UpdateType=S::UpdateType>>,
                           policy: Box<dyn Policy<StateType=Box<dyn InformationSet<
                               ActionIteratorType=Vec<S::ActionType>,
                               ActionType=S::ActionType,
                               Error: Into<SztormError<S>>,
                               Id=S::AgentId,
                               RewardType: Reward,
                               UpdateType=S::UpdateType>>>>) -> Result<(), SetupError<S>>{



    }*/
/*
    pub fn add_local_agent<Agnt: AgentRR<S>, P: Policy>(&mut self,
                                                        information_set: <Agnt as StatefulAgent>::State,
                                                        policy: <Agnt as ActingAgent>::)
                                                -> Result<(), SetupError<S>>
    //where InfSet = P::StateType
    {

        let (comm_env, comm_agent) = SyncCommEnv::<S>::new_pair();
        let id = information_set.id();
        if self.comm_endpoints.contains_key(id){
            return Err(DuplicateId(*id));
        }
        let agent = AgentGen::new(information_set, comm_agent, policy);

        self.comm_endpoints.insert(id, Box::new(comm_env));
        self.local_agents.insert(id, Box::new(agent));
        Ok(())

    }

 */




}
/*
impl<S: ProtocolSpecification,
    B: EnvironmentBuilder<ProtocolSpec=S>,
    P: Policy<StateType: InformationSet<Id=S::AgentId>>>  RoundRobinModel<S, B>{

    pub fn add_local_agent(&mut self, information_set: P::StateType, policy: P)
        -> Result<(), SetupError<S>>
    {

        let (comm_env, comm_agent) = SyncCommEnv::<S>::new_pair();
        let id = information_set.id();
        if self.comm_endpoints.contains_key(id){
            return Err(DuplicateId(*id));
        }
        let agent = AgentGen::new(information_set, comm_agent, policy);

        self.comm_endpoints.insert(id, Box::new(comm_env));
        self.local_agents.insert(id, Box::new(agent));
        Ok(())

    }
}


 */
/*
impl<S: ProtocolSpecification, E: EnvironmentRR<S>> Index<S::AgentId> for RoundRobinModel<S,E>{
    type Output = Option<Box<dyn AgentRR<S> + Send>>;

    fn index(&self, index: S::AgentId) -> &Self::Output {
        &self.agents.get(&index)
    }
}*/

