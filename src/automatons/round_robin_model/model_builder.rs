
use std::collections::{HashMap};



use crate::automatons::rr::{RoundRobinModel};
use crate::{ActionProcessor, EnvironmentState, GenericEnvironmentBuilder, EnvCommEndpoint, EnvironmentBuilderTrait, AgentAuto, EnvironmentStateUniScore, ActionProcessorPenalising};
use crate::error::{SetupError};

use crate::protocol::{DomainParameters};

pub struct RoundRobinModelBuilder<Spec: DomainParameters, EnvState: EnvironmentStateUniScore<Spec>,
ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec> >{
    env_builder: GenericEnvironmentBuilder<Spec, EnvState, ProcessAction, Comm>,
    //_spec: PhantomData<S>,

    //environment_state: E,
    local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec> + Send>>,
    /*comm_endpoints: HashMap<Spec::AgentId,
        Box<dyn CommEndpoint<
            OutwardType=EnvMessage<Spec>,
            InwardType=AgentMessage<Spec>,
            Error=CommError>>
        >

     */




}

#[allow(clippy::borrowed_box)]
impl<Spec: DomainParameters, EnvState: EnvironmentStateUniScore<Spec>,
ProcessAction: ActionProcessorPenalising<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>>
RoundRobinModelBuilder<Spec, EnvState, ProcessAction, Comm>
//where <<B as EnvironmentBuilder>::Environment as CommunicatingEnv>::AgentId> = <<>>
{
    pub fn new() -> Self{
        Self{ env_builder: GenericEnvironmentBuilder::new(), local_agents:HashMap::new() }
    }
    
    pub fn with_env_state(mut self, environment_state: EnvState)
        -> Result<Self, SetupError<Spec>>{
        self.env_builder = self.env_builder.with_state(environment_state)?;
        Ok(self)
    }
    pub fn with_env_action_process_fn(mut self, process_fn: ProcessAction) -> Result<Self, SetupError<Spec>>{
        self.env_builder = self.env_builder.with_processor(process_fn)?;
        Ok(self)
    }
    pub fn get_agent(&self, s: &Spec::AgentId) -> Option<&Box<dyn AgentAuto<Spec> + Send>>{
        self.local_agents.get(s)


    }

    //pub fn add_local_agent<A: AgentRR<Spec>>(&mut self, agent: A, )
    //pub fn with_local_agent<A: AgentRR<Spec>>(self, agent: A, env_comm: dyn CommEndpoint)
    pub fn with_local_agent(mut self,
                            agent: Box<dyn AgentAuto<Spec> + Send>,
                            env_comm: Comm)
                            -> Result<Self, SetupError<Spec>>{
        //if self.local_agents.contains_key(agent.as_ref().id())
        //self.comm_endpoints.insert(*agent.as_ref().id(), env_comm);
        self.env_builder = self.env_builder.add_comm(agent.as_ref().id(), env_comm)?;
        self.local_agents.insert(*agent.as_ref().id(), agent);

        Ok(self)
    }

    pub fn with_remote_agent(mut self, agent_id: Spec::AgentId,
        env_comm: Comm) -> Result<Self, SetupError<Spec>>{

        if self.local_agents.contains_key(&agent_id){
            self.local_agents.remove(&agent_id);
        }
        //self.comm_endpoints.insert(agent_id, env_comm);
        self.env_builder = self.env_builder.add_comm(&agent_id, env_comm)?;
        Ok(self)
    }

    pub fn build(self) -> Result<RoundRobinModel<Spec, EnvState, ProcessAction, Comm>, SetupError<Spec>>{
        Ok(RoundRobinModel::new(self.env_builder.build()?, self.local_agents))
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

impl<Spec: DomainParameters, EnvState: EnvironmentStateUniScore<Spec>,
ProcessAction: ActionProcessorPenalising<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>> Default for RoundRobinModelBuilder<Spec, EnvState, ProcessAction, Comm> {
    fn default() -> Self {
        Self::new()
    }
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

