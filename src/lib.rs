//! # Sztorm
//!
//! __Licence:__ MIT
//!
//! The objective of this crate is to provide framework for simulating interactions
//! between players. Players may follow defined policies or my be run in reinforcement
//! learning sessions to develop their policies.

//! # Minimal example
//! ```
//! use std::collections::HashMap;
//! use std::thread;
//! use sztorm::agent::{AgentGenT, AutomaticAgent, AutomaticAgentRewarded, RandomPolicy};
//! use sztorm::comm::SyncCommEnv;
//! use sztorm::demo::DemoAgentID::Blue;
//! use sztorm::demo::{DemoInfoSet, DemoParams, DemoState};
//! use sztorm::env::generic::{HashMapEnvT};
//! use sztorm::env::{RoundRobinEnvironment, RoundRobinUniversalEnvironment, TracingEnv};
//! let bandits = vec![5.0, 5.5, 6.0];
//! let number_of_bandits = bandits.len();
//! let state = DemoState::new(bandits, 10);
//! let (comm_env, comm_agent) = SyncCommEnv::new_pair();
//! let mut env_comms = HashMap::new();
//! env_comms.insert(Blue, comm_env);
//! let mut environment = HashMapEnvT::new(state, env_comms);
//! let blue_info_set = DemoInfoSet::new(number_of_bandits);
//! let mut agent = AgentGenT::new(Blue, blue_info_set, comm_agent, RandomPolicy::<DemoParams, DemoInfoSet>::new());
//!
//! thread::scope(|s|{
//!     s.spawn(||{
//!         environment.run_round_robin_uni_rewards().unwrap();
//!     });
//!     s.spawn(||{
//!         agent.run_rewarded().unwrap();
//!     });
//! });
//!
//! assert_eq!(environment.trajectory().list().len(), 10);
//! ```

/// Traits for managing state (or information set) of the game.
pub mod state;
/// Traits and generic implementations of agent (player).
pub mod agent;
/// Generic structs used in communication between _agents_ and _environment_.
pub mod domain;
/// Traits and basic implementation for communication driving structs.
pub mod comm;
/// Structures used for error handling in framework.
pub mod error;
/// Traits and generic implementations for game controlling environment.
pub mod env;
pub mod demo;
//mod map;


//pub use map::*;