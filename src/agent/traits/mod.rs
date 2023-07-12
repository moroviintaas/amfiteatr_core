mod communication_agent;
mod stateful_agent;
mod automatic;
mod rewarded_agent;
mod tracing_agent;
mod policy_agent;
mod reset_agent;

pub use communication_agent::*;
pub use stateful_agent::*;
pub use automatic::*;
pub use rewarded_agent::*;
pub use tracing_agent::*;
pub use reset_agent::*;
pub use policy_agent::*;
use crate::protocol::DomainParameters;

pub trait Agent<DP: DomainParameters>{
    fn id(&self) -> DP::AgentId;
}