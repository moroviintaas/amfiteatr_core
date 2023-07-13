mod builder;
mod automatons;
pub mod generic;
mod traits;


pub use traits::*;
pub use crate::state::env::*;
pub use builder::*;
pub use automatons::rr::*;




//impl <Agnt: IdentifiableAgent, Spec> EnvironmentRR
