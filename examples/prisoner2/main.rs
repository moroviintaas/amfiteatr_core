pub mod domain;
pub mod agent;
pub mod env;
pub mod common;

use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter};
use sztorm::Action;
use sztorm::agent::Policy;
use sztorm::error::{InternalGameError, SztormError};
use sztorm::protocol::DomainParameters;
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::{State, StateUpdate};











fn main(){
    println!("Hello prisoners;")
}