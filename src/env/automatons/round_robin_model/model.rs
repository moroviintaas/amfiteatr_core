use std::collections::HashMap;
use std::thread;
use log::{error, info};
use crate::protocol::{DomainParameters};
use crate::agent::AutomaticAgent;
use crate::env::{ActionProcessor, EnvironmentStateUniScore, GenericEnvironment};
use crate::env::automatons::rr::RoundRobinUniversalEnvironment;
use crate::comm::EnvCommEndpoint;
use crate::error::SztormError;

pub struct RoundRobinModel<Spec: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<Spec>,
    ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>>{
    environment: GenericEnvironment<Spec, EnvState, ProcessAction, Comm>,
    local_agents: HashMap<Spec::AgentId, Box<dyn AutomaticAgent<Spec> + Send>>,
}

impl<Spec: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<Spec>,
    ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>> RoundRobinModel<Spec, EnvState, ProcessAction, Comm>{
    pub fn new(environment: GenericEnvironment<Spec, EnvState, ProcessAction, Comm>, local_agents: HashMap<Spec::AgentId,Box<dyn AutomaticAgent<Spec> + Send>>) -> Self{
        Self{environment, local_agents}
    }

    /*
    fn agent(&self, agent: &Spec::AgentId) -> Option<&Box<dyn AgentAuto<Spec> + Send>>{
        self.local_agents.get(agent)
    }

    fn agent_mut(&mut self, agent: &Spec::AgentId) -> Option<&mut Box<dyn AgentAuto<Spec> + Send>>{
        self.local_agents.get_mut(agent)
    }
     */


    pub fn play(&mut self) -> Result<(), SztormError<Spec>>{
        let mut agent_collectors = HashMap::<Spec::AgentId, std::sync::mpsc::Receiver<Box<dyn AutomaticAgent<Spec> + Send>>>::new();
        //let mut join_handles = Vec::with_capacity(self.local_agents.len());

        //let moved_agents = std::mem::take(self.local_agents);
        /*thread::scope(|s|{
            for (id, agent) in self.local_agents.drain().take(1){
                let (agent_return_sender, agent_return_receiver) = std::sync::mpsc::channel();
                agent_collectors.insert(id, agent_return_receiver);
                s.spawn(||{
                    agent_return_sender
                })

            }
        });

         */

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