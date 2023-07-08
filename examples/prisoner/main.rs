/*
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, write};
use sztorm::Action;
use sztorm::error::{InternalGameError, SztormError};
use sztorm::protocol::DomainParameters;
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::{State, StateUpdate};


use crate::PrisonerAction::{Betray, Cover};
//---------------------------------------------------------------
// Setup action
#[derive(Copy, Clone, Debug)]
enum PrisonerAction{
    Betray,
    Cover
}


impl Display for PrisonerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Action for PrisonerAction{}
//---------------------------------------------------------------

type PrisonerReward = i32;



#[derive(thiserror::Error, Debug, PartialEq, Clone)]
struct PrisonerError;
impl Display for PrisonerError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrisonerError")
    }
}

impl Into<SztormError<PrisonerDomain>> for PrisonerError {
    fn into(self) -> SztormError<PrisonerDomain> {
        SztormError::Game(self)
    }
}



impl InternalGameError<PrisonerDomain> for PrisonerError{

}

#[derive(Clone, Debug)]
struct PrisonerDomain;

#[derive(Debug, Copy, Clone)]
struct PrisonerCommit(PrisonerAction, PrisonerAction);
impl StateUpdate for PrisonerCommit{}
type PrisonerId = u8;

impl DomainParameters for PrisonerDomain{
    type ActionType = PrisonerAction;
    type GameErrorType = PrisonerError;
    type UpdateType = PrisonerCommit;
    type AgentId = PrisonerId;
    type UniversalReward = PrisonerReward;
}

//---------------------------------------------------------------
// Setup state
struct RewardTable{
    pub reward_table: HashMap<(PrisonerAction, PrisonerAction),PrisonerReward>,
}

#[derive(Clone, Debug)]
struct PrisonerState{
    previous_actions: Vec<(PrisonerAction, PrisonerAction)>,
    reward_table: RewardTable,
    //finished: bool,
    player_id: PrisonerId,

}

impl State<PrisonerDomain> for PrisonerState {
    fn update(&mut self, update: PrisonerCommit) -> Result<(), PrisonerError> {
        self.previous_actions.push(update);
        Ok(())
    }

}



impl InformationSet<PrisonerDomain> for PrisonerState{
    type ActionIteratorType = [PrisonerAction;2];

    fn available_actions(&self) -> Self::ActionIteratorType {
        [Betray, Cover]
    }

    fn id(&self) -> &PrisonerId {
        &self.player_id
    }

    fn is_action_valid(&self, action: &PrisonerAction) -> bool {
        true
    }
}

impl ScoringInformationSet<PrisonerDomain> for PrisonerState{
    type RewardType = PrisonerReward;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.previous_actions.iter().fold(0, |acc, x|{
            acc + self.reward_table[x]
        })
    }
}



 */


fn main(){
    println!("Hello prisoners;")
}