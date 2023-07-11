mod communication_agent;
mod stateful_agent;
mod automatic;
mod rewarded_agent;
mod tracing_agent;
pub mod policy_agent;

pub use communication_agent::*;
pub use stateful_agent::*;
pub use automatic::*;
pub use rewarded_agent::*;
pub use tracing_agent::*;
use crate::protocol::DomainParameters;

pub trait Agent<DP: DomainParameters>{
    fn id(&self) -> DP::AgentId;
}