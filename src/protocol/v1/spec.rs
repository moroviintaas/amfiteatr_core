use std::fmt::{Debug, Display};
use crate::action::Action;
use crate::agent::AgentIdentifier;
use crate::error::{InternalGameError};
use crate::state::StateUpdate;

pub trait ProtocolSpecification: Clone + Debug{
    type ActionType: Action + Display;
    type GameErrorType: InternalGameError<Self> + Clone + Eq + PartialEq + Debug + Display ;
    type UpdateType: StateUpdate;
    type AgentId: AgentIdentifier;
}