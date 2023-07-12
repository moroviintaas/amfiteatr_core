use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter};
use sztorm::Action;
use sztorm::agent::Policy;
use sztorm::error::{InternalGameError, SztormError};
use sztorm::protocol::DomainParameters;
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::{State, StateUpdate};


use crate::PrisonerAction::{Betray, Cover};
//---------------------------------------------------------------
// Setup action
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrisonerAction{
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
pub enum PrisonerError{
    #[error("Performed different action (chosen: {chosen:?}, logged: {logged:?})")]
    DifferentActionPerformed{
        chosen: PrisonerAction,
        logged: PrisonerAction
    },
    #[error("Environment logged action {0}, but none was performed")]
    NoLastAction(PrisonerAction)
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
//pub type RewardTable= HashMap<(PrisonerAction, PrisonerAction),PrisonerReward>;
#[derive(Debug, Copy, Clone)]
pub struct RewardTable{
    pub cover_v_cover: PrisonerReward,
    pub cover_v_betray: PrisonerReward,
    pub betray_v_cover: PrisonerReward,
    pub betray_v_betray: PrisonerReward
}

impl RewardTable{

    pub fn reward(&self, action: PrisonerAction, other_action: PrisonerAction) -> PrisonerReward{
        match (action, other_action){
            (Cover, Cover) => self.cover_v_cover,
            (Cover, Betray) => self.cover_v_betray,
            (Betray, Cover) => self.betray_v_cover,
            (Betray, Betray) => self.betray_v_betray
        }
    }
}




#[derive(Clone, Debug)]
struct PrisonerState{
    previous_actions: Vec<PrisonerCommit>,
    reward_table: RewardTable,
    last_action: Cell<Option<PrisonerAction>>

}

impl PrisonerState{
    pub fn new(reward_table: RewardTable) -> Self{
        Self{reward_table, last_action: Cell::new(None), previous_actions: Vec::new()}
    }

    pub fn _select_action(&self, action: PrisonerAction){
        self.last_action.set(Some(action));
    }

    pub fn previous_actions(&self) -> &Vec<PrisonerCommit>{
        &self.previous_actions
    }
}

impl State<PrisonerDomain> for PrisonerState {
    fn update(&mut self, update: PrisonerCommit) -> Result<(), PrisonerError> {
        let last = self.last_action.get();
        if let Some(my_action) = last{
            if my_action == update.0{
                self.previous_actions.push(update);
                self.last_action.set(None);
                Ok(())
            } else{
                Err(PrisonerError::DifferentActionPerformed {chosen: my_action, logged: update.0})
            }
        } else {
            Err(PrisonerError::NoLastAction(update.0))
        }
    }
}

pub struct CoverPolicy{}

impl Policy<PrisonerDomain> for CoverPolicy{
    type StateType = PrisonerState;

    fn select_action(&self, state: &Self::StateType) -> Option<PrisonerAction> {
        state._select_action(Cover);
        Some(Cover)
    }
}

pub struct Forgive1Policy{}

impl Policy<PrisonerDomain> for Forgive1Policy{
    type StateType = PrisonerState;

    fn select_action(&self, state: &Self::StateType) -> Option<PrisonerAction> {
        let enemy_betrayals = state.previous_actions().iter().filter(| &step|{
            step.1 == Betray
        }).count();
        if enemy_betrayals > 1 {
            state._select_action(Betray);
            Some(Betray)
        } else {
            state._select_action(Cover);
            Some(Cover)
        }

    }
}







impl InformationSet<PrisonerDomain> for PrisonerState{
    type ActionIteratorType = [PrisonerAction;2];

    fn available_actions(&self) -> Self::ActionIteratorType {
        [Betray, Cover]
    }


    fn is_action_valid(&self, _action: &PrisonerAction) -> bool {
        true
    }
}

impl ScoringInformationSet<PrisonerDomain> for PrisonerState{
    type RewardType = PrisonerReward;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.previous_actions.iter().fold(0, |acc, x|{
            acc + self.reward_table.reward(x.0, x.1)
        })
    }
}






fn main(){
    println!("Hello prisoners;")
}