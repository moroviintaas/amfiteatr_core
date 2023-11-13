
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{debug, error};
use crate::domain::{DomainParameters};
use crate::agent::AutomaticAgent;
use crate::env::{BroadcastingEnv, CommunicatingEnv, EnvironmentStateUniScore, EnvironmentWithAgents, RoundRobinUniversalEnvironment, ScoreEnvironment};
use crate::comm::EnvCommEndpoint;
use crate::env::generic::{HashMapEnv};
use crate::error::{AmfiError, CommunicationError, WorldError};

pub struct GenericModel<
    DP: DomainParameters + 'static,
    Env: EnvironmentWithAgents<DP>
        + BroadcastingEnv<DP>
        + CommunicatingEnv<DP, CommunicationError=CommunicationError<DP>>
>{
    environment: Env,
    local_agents: HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>,
}

impl<
    DP: DomainParameters + 'static,
    Env: EnvironmentWithAgents<DP>
        + BroadcastingEnv<DP>
        + CommunicatingEnv<DP, CommunicationError=CommunicationError<DP>>
>GenericModel<DP, Env>{
    pub fn new(environment: Env, local_agents: HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP>  + Send >>>) -> Self{
        Self{environment, local_agents}
    }




    pub fn play<F: Fn(&mut Env) -> Result<(), AmfiError<DP>>>(&mut self, environment_run: F) -> Result<(), AmfiError<DP>>{

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
            //self.environment.run_round_robin_uni_rewards().map_err(|e|{
            environment_run(&mut self.environment).map_err(|e|{
                error!("Environment run error: {e:}");
                e
            }).unwrap();

        });

        Ok(())

    }
    pub fn play_rr_uni_reward(&mut self) -> Result<(), AmfiError<DP>>
    where Env: RoundRobinUniversalEnvironment<DP>{
        self.play(| env| env.run_round_robin_uni_rewards())
    }



    pub fn env(&self) -> &Env{
        &self.environment
    }
    pub fn local_agents(&self) -> &HashMap<DP::AgentId, Arc<Mutex<dyn AutomaticAgent<DP> + Send>>>{
        &self.local_agents
    }
}
