
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use crate::agent::{Agent, AgentGen, AutomaticAgent, Policy};
use crate::env::{EnvironmentBuilderTrait, EnvironmentStateUniScore};
use crate::env::automatons::rr::RoundRobinModel;
use crate::comm::{EnvCommEndpoint, SyncCommEnv};
use crate::env::generic::{GenericEnvironmentBuilder};
use crate::error::{SetupError};

use crate::protocol::{DomainParameters};
use crate::state::agent::ScoringInformationSet;

pub struct RoundRobinModelBuilder<
    DP: DomainParameters,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP> >{
    env_builder: GenericEnvironmentBuilder<DP, EnvState,  Comm>,
    local_agents: HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>,

}


impl<
    DP: DomainParameters,
    EnvState: EnvironmentStateUniScore<DP>>
RoundRobinModelBuilder<DP, EnvState,  SyncCommEnv<DP>>
{
    pub fn with_local_generic_agent<P: Policy<DP> + 'static>(
        mut self,
        id: DP::AgentId,
        initial_state: <P as Policy<DP>>::StateType,
        policy: P)
        -> Result<Self, SetupError<DP>>
        where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

        let (comm_env, comm_agent) = SyncCommEnv::new_pair();
        let agent = AgentGen::new(id, initial_state, comm_agent, policy);
        self.env_builder = self.env_builder.add_comm(&agent.id(), comm_env)?;
        self.local_agents.insert(agent.id(), Arc::new(Mutex::new(agent)));
        Ok(self)

    }
}
/*
impl<
    DP: DomainParameters,
    EnvState: EnvironmentStateUniScore<DP>>
RoundRobinModelBuilder<DP, EnvState,  Box<dyn EnvCommEndpoint<DP, Error=CommError<DP>, InwardType=AgentMessage<DP>, OutwardType=EnvMessage<DP>>>>{
    pub fn with_local_generic_agent<P: Policy<DP> + 'static>(
        mut self,
        id: DP::AgentId,
        initial_state: <P as Policy<DP>>::StateType,
        policy: P)
        -> Result<Self, SetupError<DP>>{

        let (comm_env, comm_agent) = SyncCommEnv::new_pair();
        let agent = AgentGen::new(id, initial_state, comm_agent, policy);
        self.env_builder = self.env_builder.add_comm(&agent.id(), Box::new(comm_env))?;
        self.local_agents.insert(agent.id(), Box::new(agent));
        Ok(self)

    }
}*/


#[allow(clippy::borrowed_box)]
impl<
    DP: DomainParameters,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP>>
RoundRobinModelBuilder<DP, EnvState,  Comm>{
    pub fn new() -> Self{
        Self{ env_builder: GenericEnvironmentBuilder::new(), local_agents:HashMap::new() }
    }
    
    pub fn with_env_state(mut self, environment_state: EnvState)
        -> Result<Self, SetupError<DP>>{
        self.env_builder = self.env_builder.with_state(environment_state)?;
        Ok(self)
    }
    /*
    pub fn with_env_action_process_fn(mut self, process_fn: ProcessAction) -> Result<Self, SetupError<DP>>{
        self.env_builder = self.env_builder.with_processor(process_fn)?;
        Ok(self)
    }*/
    pub fn get_agent(&self, s: &DP::AgentId) -> Option<&Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>{
        self.local_agents.get(s)


    }

    pub fn add_local_agent(mut self,
                           agent: Arc<Mutex<dyn AutomaticAgent<DP> + Send>>,
                           env_comm: Comm)
                           -> Result<Self, SetupError<DP>>{

        let agent_guard = agent.as_ref().lock().unwrap();
        let id = agent_guard.id();
        std::mem::drop(agent_guard);
        self.env_builder = self.env_builder.add_comm(&id, env_comm)?;
        self.local_agents.insert(id, agent);

        Ok(self)
    }



    pub fn with_remote_agent(mut self, agent_id: DP::AgentId,
                             env_comm: Comm) -> Result<Self, SetupError<DP>>{

        if self.local_agents.contains_key(&agent_id){
            self.local_agents.remove(&agent_id);
        }
        //self.comm_endpoints.insert(agent_id, env_comm);
        self.env_builder = self.env_builder.add_comm(&agent_id, env_comm)?;
        Ok(self)
    }

    pub fn build(self) -> Result<RoundRobinModel<DP, EnvState, Comm>, SetupError<DP>>{
        Ok(RoundRobinModel::new(self.env_builder.build()?, self.local_agents))
    }




}

impl<Spec: DomainParameters, EnvState: EnvironmentStateUniScore<Spec>,
 Comm: EnvCommEndpoint<Spec>> Default for RoundRobinModelBuilder<Spec, EnvState, Comm> {
    fn default() -> Self {
        Self::new()
    }
}
