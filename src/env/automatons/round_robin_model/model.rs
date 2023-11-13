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
use crate::error::{AmfiError, WorldError};

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




    pub fn play(&mut self) -> Result<(), AmfiError<DP>>{

        thread::scope(|s|{
            let mut handlers = HashMap::new();
            for (id, agent) in self.local_agents.iter(){
                let arc_agent = agent.clone();


                let handler = s.spawn( move ||{
                    debug!("Spawning thread for agent {}", id);
                    let mut guard = arc_agent.lock().or_else(|_|Err(WorldError::<DP>::AgentMutexLock)).unwrap();
                    let id = guard.id().clone();
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
