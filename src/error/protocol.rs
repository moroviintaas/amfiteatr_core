use thiserror::Error;
use crate::agent::AgentIdentifier;
use crate::error::TurError;
use crate::protocol::ProtocolSpecification;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum ProtocolError<Spec: ProtocolSpecification>{
    #[error("lost contact with {:}", .0)]
    BrokenComm(Spec::AgentId),
    #[error("agent {:} attempted to move on turn of {:}", .0, .1)]
    ViolatedOrder(Spec::AgentId, Spec::AgentId),
    #[error("agent {:} called to move, however called states that {:} should move this time", .0, .1)]
    OrderDesync(Spec::AgentId, Spec::AgentId),
    #[error("agent {:} received kill", .0)]
    ReceivedKill(Spec::AgentId),
    #[error("agent {:} has no possible action", .0)]
    NoPossibleAction(Spec::AgentId),
}

impl<Spec: ProtocolSpecification> From<ProtocolError<Spec>> for TurError<Spec>{
    fn from(value: ProtocolError<Spec>) -> Self {
        Self::ProtocolError(value)
    }
}