use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use log::{error, info};
use crate::protocol::{EnvMessage, ProtocolSpecification};
use crate::{ActionProcessor, AutomaticEnvironment, CommunicatingEnv, EnvironmentState, GenericEnvironment, StatefulEnvironment, EnvCommEndpoint};
use crate::automatons::rr::{AgentAuto, EnvironmentRR};
use crate::error::SztormError;

pub struct RoundRobinModel<Spec: ProtocolSpecification + 'static,
    EnvState: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>>{
    environment: GenericEnvironment<Spec, EnvState, ProcessAction, Comm>,
    local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec> + Send>>,
}

impl<Spec: ProtocolSpecification + 'static,
    EnvState: EnvironmentState<Spec>,
    ProcessAction: ActionProcessor<Spec, EnvState>, Comm: EnvCommEndpoint<Spec>> RoundRobinModel<Spec, EnvState, ProcessAction, Comm>{
    pub fn new(environment: GenericEnvironment<Spec, EnvState, ProcessAction, Comm>, local_agents: HashMap<Spec::AgentId,Box<dyn AgentAuto<Spec> + Send>>) -> Self{
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

        for (id, mut agent) in self.local_agents.drain().into_iter(){
            //self.
            let (agent_return_sender, agent_return_receiver) = std::sync::mpsc::channel();
            agent_collectors.insert(id, agent_return_receiver);
            thread::spawn(move ||{
                //if let Ok(mut agent_guard) = agent.lock(){
                    agent.run_rr().map_err(|e|{
                        error!("Agent {id:} encountered error: {e:}")
                    }).unwrap();
                //}
                
                agent_return_sender.send(agent).expect("Error sending back agent for collection");
            });

            

        }

        info!("Collector HashMap len: {}", agent_collectors.len());
        self.environment.env_run_rr().map_err(|e|{
            error!("Environment run error: {e:}");
            e
        }).unwrap();

        for (id, col) in agent_collectors.drain().into_iter(){
            let agent = col.recv().unwrap();
            self.local_agents.insert(id, agent);
        }

        Ok(())
        //todo!()
    }
}