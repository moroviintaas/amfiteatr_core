mod communication_agent;
mod stateful_agent;
mod automatic;
mod rewarded_agent;
mod tracing_agent;
mod policy_agent;
mod reset_agent;
mod internal_rewarded_agent;

pub use communication_agent::*;
pub use stateful_agent::*;
pub use automatic::*;
pub use rewarded_agent::*;
pub use tracing_agent::*;
pub use reset_agent::*;
pub use policy_agent::*;
pub use internal_rewarded_agent::*;
use crate::domain::DomainParameters;

/// Basic Agent trait, basic agent constrains require it to have id.
/// Although agent may somehow change it, it is important to keep environment updated.
/// When environment uses communication endpoint to /send to/recv from/ agent it should
/// communicate with just this meant agent not any other.
/// Id's of agents should be unique in the game world.
pub trait Agent<DP: DomainParameters>{
    fn id(&self) -> &DP::AgentId;
    fn change_id(&mut self, new_id: DP::AgentId);
}