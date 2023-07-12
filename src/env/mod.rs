mod communication_env;
mod stateful_env;
mod environment_with_agents;
mod builder;
//mod automatic;
mod domain_env;
mod growing_environment;
mod contructed_environment;
mod score_environment;
mod automatons;
pub mod generic;

pub use communication_env::*;
pub use stateful_env::*;
pub use environment_with_agents::*;
pub use crate::state::env::*;
pub use builder::*;
pub use automatons::rr::*;
pub use domain_env::*;
pub use growing_environment::*;
pub use contructed_environment::*;
pub use score_environment::*;



//impl <Agnt: IdentifiableAgent, Spec> EnvironmentRR
