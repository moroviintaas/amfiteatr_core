use std::collections::HashMap;
use std::thread;
use log::error;
use crate::protocol::{EnvMessage, ProtocolSpecification};
use crate::{ActionProcessingFunction, AutomaticEnvironment, CommunicatingEnv, EnvironmentState, GenericEnvironment, StatefulEnvironment};
use crate::automatons::rr::{AgentAuto, EnvironmentRR};
use crate::error::SztormError;

pub struct RoundRobinModel<Spec: ProtocolSpecification + 'static,
    EnvState: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, EnvState>>{
    environment: GenericEnvironment<Spec, EnvState, ProcessAction>,
    local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec> + Send>>,
}

impl<Spec: ProtocolSpecification + 'static,
    EnvState: EnvironmentState<Spec>,
    ProcessAction: ActionProcessingFunction<Spec, EnvState>> RoundRobinModel<Spec, EnvState, ProcessAction>{
    pub fn new(environment: GenericEnvironment<Spec, EnvState, ProcessAction>, local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec> + Send>>) -> Self{
        Self{environment, local_agents}
    }

    pub fn play(&mut self) -> Result<(), SztormError<Spec>>{
        let mut agent_collectors = HashMap::<Spec::AgentId, std::sync::mpsc::Receiver<Box<dyn AgentAuto<Spec> + Send>>>::new();
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

        for (id, mut agent) in self.local_agents.drain().take(1){
            let (agent_return_sender, agent_return_receiver) = std::sync::mpsc::channel();
            agent_collectors.insert(id, agent_return_receiver);
            thread::spawn(move ||{
                agent.run_rr().map_err(|e|{
                    error!("Agent {id:} encountered error: {e:}")
                }).unwrap();
                agent_return_sender.send(agent).expect("Error sending back agent for collection");
            });

        }

        self.environment.env_run_rr().map_err(|e|{
            error!("Environment run error: {e:}");
            e
        }).unwrap();

        for (id, col) in agent_collectors.drain().take(1){
            let agent = col.recv().unwrap();
            self.local_agents.insert(id, agent);
        }

        Ok(())
        //todo!()
    }
}