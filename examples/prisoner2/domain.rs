use std::fmt::{Display, Formatter};
use sztorm::Action;
use sztorm::agent::Policy;
use sztorm::error::{InternalGameError, SztormError};
use sztorm::protocol::DomainParameters;
use sztorm::state::StateUpdate;

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
//--------------------------------------


#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum PrisonerError{
    #[error("Performed different action (chosen: {chosen:?}, logged: {logged:?})")]
    DifferentActionPerformed{
        chosen: PrisonerAction,
        logged: PrisonerAction
    },
    #[error("Environment logged action {0}, but none was performed")]
    NoLastAction(PrisonerAction),
    #[error("Player: {0} played after GameOver")]
    ActionAfterGameOver(PrisonerId),
    #[error("Player: {0} played out of order")]
    ActionOutOfOrder(PrisonerId),
}


impl Into<SztormError<PrisonerDomain>> for PrisonerError {
    fn into(self) -> SztormError<PrisonerDomain> {
        SztormError::Game(self)
    }
}



impl InternalGameError<PrisonerDomain> for PrisonerError{

}


#[derive(Clone, Debug)]
pub struct PrisonerDomain;
#[derive(Debug, Copy, Clone)]
pub struct PrisonerUpdate{
    pub own_action: PrisonerAction,
    pub other_prisoner_action: PrisonerAction}
impl StateUpdate for PrisonerUpdate{}
pub type PrisonerId = u8;

pub const PRISONERS:[u8;2] = [0u8,1];

pub type PrisonerReward = i32;


impl DomainParameters for PrisonerDomain{
    type ActionType = PrisonerAction;
    type GameErrorType = PrisonerError;
    type UpdateType = PrisonerUpdate;
    type AgentId = PrisonerId;
    type UniversalReward = PrisonerReward;
}
