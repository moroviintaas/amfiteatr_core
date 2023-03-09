mod communication_env;
mod stateful_env;
mod environment;

pub use communication_env::*;
pub use stateful_env::*;
pub use environment::*;
use crate::agent::IdentifiableAgent;
use crate::automatons::rr::EnvironmentRR;


//impl <Agnt: IdentifiableAgent, Spec> EnvironmentRR