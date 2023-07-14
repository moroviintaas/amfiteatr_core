use std::fmt::{Display, Formatter};
use sztorm::Action;
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
pub struct PrisonerDomain;

#[derive(Debug, Copy, Clone)]
pub struct PrisonerCommit(pub PrisonerAction, pub PrisonerAction);
impl StateUpdate for PrisonerCommit{}
pub type PrisonerId = u8;

pub type PrisonerReward = i32;


impl DomainParameters for PrisonerDomain{
    type ActionType = PrisonerAction;
    type GameErrorType = PrisonerError;
    type UpdateType = PrisonerCommit;
    type AgentId = PrisonerId;
    type UniversalReward = PrisonerReward;
}
