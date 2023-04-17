use std::collections::HashMap;
use std::thread;
use crate::protocol::ProtocolSpecification;
use crate::{AutomaticEnvironment, CommunicatingEnv, StatefulEnvironment};
use crate::automatons::rr::{AgentAuto, EnvironmentRR};
use crate::error::SztormError;

pub struct RoundRobinModel<Spec: ProtocolSpecification, Env: EnvironmentRR<Spec>>{
    environment: Env,
    local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec, Id = Spec::AgentId> + Send>>,
}

impl <Spec: ProtocolSpecification, Env: EnvironmentRR<Spec>> RoundRobinModel<Spec, Env>{
    pub fn new(environment: Env, local_agents: HashMap<Spec::AgentId, Box<dyn AgentAuto<Spec, Id = Spec::AgentId> + Send>>) -> Self{
        Self{environment, local_agents}
    }

    pub fn play(&mut self) -> Result<(), SztormError<Spec>>{
        let mut agent_collectors = HashMap::<Spec::AgentId, std::sync::mpsc::Receiver<Box<dyn AgentAuto<Spec, Id = Spec::AgentId> + Send>>>::new();
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
                agent.run_rr().unwrap();
                agent_return_sender.send(agent).expect("Error sending back agent for collection");
            });

        }

        self.environment.env_run_rr().unwrap();

        for (id, col) in agent_collectors.drain().take(1){
            let agent = col.recv().unwrap();
            self.local_agents.insert(id, agent);
        }

        Ok(())
        //todo!()
    }
}