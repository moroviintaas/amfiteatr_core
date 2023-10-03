use thiserror::Error;
use crate::error::SztormError;
use crate::domain::DomainParameters;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum ProtocolError<Spec: DomainParameters>{
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
    #[error("agent {} has exited the game", .0)]
    PlayerExited(Spec::AgentId)
}

impl<Spec: DomainParameters> From<ProtocolError<Spec>> for SztormError<Spec>{
    fn from(value: ProtocolError<Spec>) -> Self {
        Self::Protocol(value)
    }
}