use std::error::Error;
use std::fmt::Debug;
use crate::action::Action;
use crate::agent::AgentIdentifier;
use crate::state::StateUpdate;

pub trait ProtocolSpecification: Clone + Debug{
    type ActionType: Action;
    type GameErrorType: Error + Clone + Eq + PartialEq;
    type UpdateType: StateUpdate;
    type AgentId: AgentIdentifier;
    //type ActionIteratorType: IntoIterator<Item=Action>;
}