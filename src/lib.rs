//! # Sztorm
//!
//! __Licence:__ MIT
//!
//! The objective of this crate is to provide framework for simulating interactions
//! between players. Players may follow defined policies or my be run in reinforcement
//! learning sessions to develop their policies.


/// Traits for managing state (or information set) of the game.
pub mod state;
/// Traits and generic implementations of agent (player).
pub mod agent;
mod action;
/// Generic structs used in communication between _agents_ and _environment_.
pub mod protocol;
/// Traits and basic implementation for communication driving structs.
pub mod comm;
/// Structures used for error handling in framework.
pub mod error;
/// Traits and generic implementations for game controlling environment.
pub mod env;
mod reward;
//mod map;


pub use action::*;

pub use reward::*;
//pub use map::*;