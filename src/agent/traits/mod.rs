mod communication_agent;
mod stateful_agent;
mod automatic;
mod rewarded_agent;
mod tracing_agent;
mod policy_agent;
mod reset_agent;
mod internal_rewarded_agent;
mod agent_with_id;
mod list_players;
mod learning_agent;

pub use communication_agent::*;
pub use stateful_agent::*;
pub use automatic::*;
pub use rewarded_agent::*;
pub use tracing_agent::*;
pub use reset_agent::*;
pub use policy_agent::*;
pub use internal_rewarded_agent::*;
pub use agent_with_id::*;
pub use list_players::*;
pub use learning_agent::*;
/*
/// Basic Agent trait, basic agent constrains require it to have id.
/// Although agent may somehow change it, it is important to keep environment updated.
/// When environment uses communication endpoint to /send to/recv from/ agent it should
/// communicate with just this meant agent not any other.
/// Id's of agents should be unique in the game world.
pub trait Agent<DP: DomainParameters>{
    fn id(&self) -> &DP::AgentId;
    fn change_id(&mut self, new_id: DP::AgentId);
}

impl<DP: DomainParameters, A: Agent<DP>> Agent<DP> for Mutex<A>{
    fn id(&self) -> &DP::AgentId {
        self.id()
    }

    fn change_id(&mut self, new_id: DP::AgentId) {
        let mut guard = self.lock().unwrap();
        guard.change_id(new_id);
        std::mem::drop(guard);
    }

}

impl<DP: DomainParameters, A: Agent<DP>> Agent<DP> for Box<A>{
    fn id(&self) -> &DP::AgentId {
        self.id()
    }

    fn change_id(&mut self, new_id: DP::AgentId) {
        self.change_id(new_id)
    }

}

 */