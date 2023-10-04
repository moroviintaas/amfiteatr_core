use std::fmt::{Debug, Display};
use crate::domain::action::Action;
use crate::agent::AgentIdentifier;
use crate::domain::Reward;
use crate::error::{InternalGameError};
//use crate::state::StateUpdate;

pub trait DomainParameters: Clone + Debug + Send + Sync + 'static{
    type ActionType: Action + Display;
    type GameErrorType: InternalGameError<Self> + Clone + PartialEq + Debug + Display + Send;
    type UpdateType: Debug + Send + Clone ;
    type AgentId: AgentIdentifier;
    type UniversalReward: Reward;
}