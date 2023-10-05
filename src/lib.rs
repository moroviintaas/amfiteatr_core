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
//! use sztorm::agent::{AgentGen, AgentGenT, AutomaticAgent, AutomaticAgentRewarded, EnvRewardedAgent, RandomPolicy};
//! use sztorm::comm::SyncCommEnv;
//! use sztorm::demo::{DemoInfoSet, DemoParams, DemoState, DemoAgentID, DemoPolicySelectFirst};
//! use sztorm::env::{*, generic::*};
//!
//!
//! let bandits = vec![5.0, 11.5, 6.0];
//! let number_of_bandits = bandits.len();
//! let state = DemoState::new(bandits, 100);
//! let (comm_env_r, comm_agent_r) = SyncCommEnv::new_pair();
//! let (comm_env_b, comm_agent_b) = SyncCommEnv::new_pair();
//! let mut env_comms = HashMap::new();
//! env_comms.insert(DemoAgentID::Blue, comm_env_b);
//! env_comms.insert(DemoAgentID::Red, comm_env_r);
//! let mut environment = HashMapEnvT::new(state, env_comms);
//! let blue_info_set = DemoInfoSet::new(number_of_bandits);
//! let red_info_set = DemoInfoSet::new(number_of_bandits);
//! let mut agent_blue = AgentGenT::new(DemoAgentID::Blue, blue_info_set, comm_agent_b, RandomPolicy::<DemoParams, DemoInfoSet>::new());
//! let mut agent_red = AgentGen::new(DemoAgentID::Red, red_info_set, comm_agent_r, DemoPolicySelectFirst{});
//!
//! thread::scope(|s|{
//!     s.spawn(||{
//!         environment.run_round_robin_uni_rewards().unwrap();
//!     });
//!     s.spawn(||{
//!         agent_blue.run_rewarded().unwrap();
//!     });
//!     s.spawn(||{
//!         agent_red.run_rewarded().unwrap();
//!     });
//! });
//!
//! assert_eq!(environment.trajectory().list().len(), 200);
//! assert!(environment.actual_score_of_player(&DemoAgentID::Blue)> 10.0);
//! assert!(agent_blue.current_universal_score()> 10.0);
//! assert!(agent_red.current_universal_score()> 10.0);
//! assert!(agent_blue.current_universal_score()> agent_red.current_universal_score());
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