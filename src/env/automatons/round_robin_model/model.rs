use std::collections::HashMap;
use std::thread;
use log::{error, info};
use crate::protocol::{DomainParameters};
use crate::agent::AutomaticAgent;
use crate::env::{EnvironmentStateUniScore};
use crate::env::automatons::rr::RoundRobinUniversalEnvironment;
use crate::comm::EnvCommEndpoint;
use crate::env::generic::{HashMapEnv};
use crate::error::SztormError;

pub struct RoundRobinModel<
    DP: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP>>{
    environment: HashMapEnv<DP, EnvState,  Comm>,
    local_agents: HashMap<DP::AgentId, Box<dyn AutomaticAgent<DP> + Send>>,
}

impl<
    DP: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP>>
RoundRobinModel<DP, EnvState, Comm>{
    pub fn new(environment: HashMapEnv<DP, EnvState, Comm>, local_agents: HashMap<DP::AgentId,Box<dyn AutomaticAgent<DP> + Send>>) -> Self{
        Self{environment, local_agents}
    }




    pub fn play(&mut self) -> Result<(), SztormError<DP>>{
        let mut agent_collectors = HashMap::<DP::AgentId, std::sync::mpsc::Receiver<Box<dyn AutomaticAgent<DP> + Send>>>::new();


        for (id, mut agent) in self.local_agents.drain(){
            //self.
            let (agent_return_sender, agent_return_receiver) = std::sync::mpsc::channel();
            agent_collectors.insert(id, agent_return_receiver);
            thread::spawn(move ||{
                //if let Ok(mut agent_guard) = agent.lock(){
                    agent.run().map_err(|e|{
                        error!("Agent {id:} encountered error: {e:}")
                    }).unwrap();
                //}
                
                agent_return_sender.send(agent).expect("Error sending back agent for collection");
            });

            

        }

        info!("Collector HashMap len: {}", agent_collectors.len());
        self.environment.run_round_robin_uni_rewards().map_err(|e|{
            error!("Environment run error: {e:}");
            e
        }).unwrap();

        for (id, col) in agent_collectors.drain(){
            let agent = col.recv().unwrap();
            self.local_agents.insert(id, agent);
        }

        Ok(())
    }

    pub fn env(&self) -> &HashMapEnv<DP, EnvState,  Comm>{
        &self.environment
    }
    pub fn local_agents(&self) -> &HashMap<DP::AgentId, Box<dyn AutomaticAgent<DP> + Send>>{
        &self.local_agents
    }
}
/*
impl<Spec: ProtocolSpecification + 'static,
    EnvState: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>> Index<Spec::AgentId> for  
RoundRobinModel<Spec, EnvState, ProcessAction, Comm>{
    type Output = Box<dyn AgentAuto<Spec> + Send;

    fn index(&self, index: Spec::AgentId) -> &Self::Output {
        self.local_agents.get(index)
    }
}*/