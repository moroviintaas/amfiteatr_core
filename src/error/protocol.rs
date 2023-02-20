use thiserror::Error;
use crate::agent::AgentIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum ProtocolError<Agent: AgentIdentifier>{
    #[error("lost contact with {:}", .0)]
    BrokenComm(Agent),
    #[error("agent {:} attempted to move on turn of {:}", .0, .1)]
    ViolatedOrder(Agent, Agent),
    #[error("agent {:} called to move, however called states that {:} should move this time", .0, .1)]
    OrderDesync(Agent, Agent)
}