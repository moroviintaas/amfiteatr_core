use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{debug, error};
use crate::domain::{DomainParameters};
use crate::agent::AutomaticAgent;
use crate::env::{EnvironmentStateUniScore};
use crate::env::automatons::rr::RoundRobinUniversalEnvironment;
use crate::comm::EnvCommEndpoint;
use crate::env::generic::{HashMapEnv};
use crate::error::{SetupError, SztormError};

pub struct RoundRobinModel<
    DP: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP>>{
    environment: HashMapEnv<DP, EnvState,  Comm>,
    local_agents: HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>,
}

impl<
    DP: DomainParameters + 'static,
    EnvState: EnvironmentStateUniScore<DP>,
    Comm: EnvCommEndpoint<DP>>
RoundRobinModel<DP, EnvState, Comm>{
    pub fn new(environment: HashMapEnv<DP, EnvState, Comm>, local_agents: HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP>  + Send >>>) -> Self{
        Self{environment, local_agents}
    }




    pub fn play(&mut self) -> Result<(), SztormError<DP>>{




        /*
        et mut agent_collectors = HashMap::<DP::AgentId, std::sync::mpsc::Receiver<Box<dyn AutomaticAgent<DP> + Send>>>::new();
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

         */
        /*
        let mut handlers = HashMap::new();
        for (id, mut agent) in self.local_agents.iter(){
            let arc_agent = agent.clone();


            let handler = thread::spawn(move ||{
                debug!("Spawning thread for agent {}", id);
                let mut guard = arc_agent.lock().or_else(|_|Err(SetupError::<DP>::AgentMutexLock)).unwrap();
                let id = guard.id();
                guard.run().map_err(|e|{
                    error!("Agent {id:} encountered error: {e:}")
                }).unwrap();
            });
            handlers.insert(id, handler);


        }
        //info!("Collector HashMap len: {}", agent_collectors.len());
        self.environment.run_round_robin_uni_rewards().map_err(|e|{
            error!("Environment run error: {e:}");
            e
        })?;
        for (id, h) in handlers.into_iter(){
            h.join().or_else(|_|Err(FailedJoinAgent(*id)))?;
        }
        Ok(())

         */
        thread::scope(|s|{
            let mut handlers = HashMap::new();
            for (id, agent) in self.local_agents.iter(){
                let arc_agent = agent.clone();


                let handler = s.spawn( move ||{
                    debug!("Spawning thread for agent {}", id);
                    let mut guard = arc_agent.lock().or_else(|_|Err(SetupError::<DP>::AgentMutexLock)).unwrap();
                    let id = guard.id();
                    guard.run().map_err(|e|{
                        error!("Agent {id:} encountered error: {e:}")
                    }).unwrap();
                });
                handlers.insert(id, handler);
            }
            self.environment.run_round_robin_uni_rewards().map_err(|e|{
                error!("Environment run error: {e:}");
                e
            }).unwrap();

        });

        Ok(())

    }

    pub fn env(&self) -> &HashMapEnv<DP, EnvState,  Comm>{
        &self.environment
    }
    pub fn local_agents(&self) -> &HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>{
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