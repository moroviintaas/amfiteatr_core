use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use itertools::Itertools;
use log::LevelFilter;
use log::LevelFilter::Debug;
use sztorm::agent::{AgentGenT, AutomaticAgent, RandomPolicy, ResetAgent, StatefulAgent};
use sztorm::comm::SyncCommEnv;
use sztorm::env::generic::TracingGenericEnv;
use sztorm::env::{ResetEnvironment, RoundRobinModelBuilder, RoundRobinUniversalEnvironment, TracingEnv};
use sztorm::error::SztormError;
use crate::agent::{BetrayRatioPolicy, CoverPolicy, Forgive1Policy, PrisonerState, RandomPrisonerPolicy};
use crate::common::RewardTable;
use crate::domain::{PrisonerDomain, PrisonerError};
use crate::domain::PrisonerAction::Betray;
use crate::domain::PrisonerId::{Andrzej, Janusz};
use crate::env::PrisonerEnvState;

pub mod domain;
pub mod agent;
pub mod env;
pub mod common;


pub fn setup_logger(log_level: LevelFilter, log_file: &Option<PathBuf>) -> Result<(), fern::InitError> {
    let dispatch  = fern::Dispatch::new()

        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level);

        match log_file{
            None => dispatch.chain(std::io::stdout()),
            Some(f) => dispatch.chain(fern::log_file(f)?)
        }

        //.chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}




fn main() -> Result<(), SztormError<PrisonerDomain>>{
    println!("Hello prisoners;");
    setup_logger(LevelFilter::Info, &None).unwrap();

    let reward_table = RewardTable{
        cover_v_cover: 5,
        betray_v_cover: 10,
        betray_v_betray: 3,
        cover_v_betray: 0
    };


    let env_state = PrisonerEnvState::new(reward_table,  100);

    let (comm_env_0, comm_prisoner_0) = SyncCommEnv::new_pair();
    let (comm_env_1, comm_prisoner_1) = SyncCommEnv::new_pair();

    let mut prisoner0 = AgentGenT::new(
        Andrzej,
        PrisonerState::new(reward_table), comm_prisoner_0, CoverPolicy{});

    let mut prisoner1 = AgentGenT::new(
        Janusz,
        PrisonerState::new(reward_table), comm_prisoner_1, Forgive1Policy{});

    let mut env_coms = HashMap::new();
    env_coms.insert(Andrzej, comm_env_0);
    env_coms.insert(Janusz, comm_env_1);

    let mut env = TracingGenericEnv::new( env_state, env_coms);

    thread::scope(|s|{
        s.spawn(||{
            env.run_round_robin_uni_rewards().unwrap();
        });
        s.spawn(||{
            prisoner0.run().unwrap();
        });
        s.spawn(||{
            prisoner1.run().unwrap();
        });
    });

    println!("Scenario 2");


    /*
    let env_state = PrisonerEnvState::new(reward_table,  100);

    let (comm_env_0, comm_prisoner_0) = SyncCommEnv::new_pair();
    let (comm_env_1, comm_prisoner_1) = SyncCommEnv::new_pair();

    let mut prisoner0 = AgentGenT::new(
        Andrzej,
        PrisonerState::new(reward_table), comm_prisoner_0, RandomPrisonerPolicy{});

    let mut prisoner1 = AgentGenT::new(
        Janusz,
        PrisonerState::new(reward_table), comm_prisoner_1, BetrayRatioPolicy{});

    let mut env_coms = HashMap::new();
    env_coms.insert(Andrzej, comm_env_0);
    env_coms.insert(Janusz, comm_env_1);

    let mut env = TracingGenericEnv::new( env_state, env_coms);

     */

    env.reset(PrisonerEnvState::new(reward_table,  100));
    let mut prisoner0 = prisoner0.do_change_policy(RandomPrisonerPolicy{});
    let mut prisoner1 = prisoner1.do_change_policy(BetrayRatioPolicy{});
    prisoner0.reset(PrisonerState::new(reward_table));
    prisoner1.reset(PrisonerState::new(reward_table));

    thread::scope(|s|{
        s.spawn(||{
            env.run_round_robin_uni_rewards().unwrap();
        });
        s.spawn(||{
            prisoner0.run().unwrap();
        });
        s.spawn(||{
            prisoner1.run().unwrap();
        });
    });

    let prisoner0_betrayals = prisoner0.state().count_actions(Betray);
    let prisoner1_betrayals = prisoner1.state().count_actions(Betray);

    println!("Prisoner 0 betrayed {:?} times and Prisoner 1 betrayed {:?} times.", prisoner0_betrayals, prisoner1_betrayals);

    for elem in env.trajectory().list(){
        println!("{}", elem);
    }



    Ok(())
}