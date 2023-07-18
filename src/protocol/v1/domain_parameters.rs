use std::fmt::{Debug, Display};
use crate::action::Action;
use crate::agent::AgentIdentifier;
use crate::error::{InternalGameError};
use crate::Reward;
use crate::state::StateUpdate;

pub trait DomainParameters: Clone + Debug + Send + Sync + 'static{
    type ActionType: Action + Display;
    type GameErrorType: InternalGameError<Self> + Clone + PartialEq + Debug + Display + Send;
    type UpdateType: StateUpdate;
    type AgentId: AgentIdentifier;
    type UniversalReward: Reward;
}