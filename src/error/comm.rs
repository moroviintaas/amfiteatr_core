use std::sync::mpsc::{RecvError, SendError, TryRecvError, TrySendError};
use thiserror::Error;

use crate::error::SztormError;
use crate::domain::DomainParameters;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum CommError<Spec: DomainParameters>{
    #[error("Send Error to {0}")]
    SendError(Spec::AgentId),
    #[error("Send Error")]
    SendErrorUnspecified,
    #[error("Broadcast Send Error (on {0})")]
    BroadcastSendError(Spec::AgentId),
    #[error("Broadcast Send Error")]
    BroadcastSendErrorUnspecified,
    #[error("TrySend Error to {0}")]
    TrySendError(Spec::AgentId),
    #[error("TrySend Error")]
    TrySendErrorUnspecified,
    #[error("Recv Error from {0}")]
    RecvError(Spec::AgentId),
    #[error("Recv Error")]
    RecvErrorUnspecified,
    #[error("TryRecv Error (empty) from {0}")]
    TryRecvEmptyError(Spec::AgentId),
    #[error("TryRecv Error (empty")]
    TryRecvErrorEmptyUnspecified,
    #[error("TryRecv Error (disconnected")]
    TryRecvErrorDisconnectedUnspecified,
    #[error("TryRecv Error (disconnected) from {0}")]
    TryRecvDisconnectedError(Spec::AgentId),
    #[error("Serialize Error")]
    SerializeError,
    #[error("Deserialize Error")]
    DeserializeError,
    #[error("No such connection")]
    NoSuchConnection,

}

impl<Spec: DomainParameters> CommError<Spec>{

    pub fn specify_id(self, id: Spec::AgentId) -> Self{
        match self{
            CommError::SendErrorUnspecified => Self::SendError(id),
            CommError::BroadcastSendErrorUnspecified => Self::BroadcastSendError(id),
            CommError::TrySendErrorUnspecified => Self::TrySendError(id),
            CommError::RecvErrorUnspecified => Self::RecvError(id),
            CommError::TryRecvErrorEmptyUnspecified => Self::TryRecvEmptyError(id),
            CommError::TryRecvErrorDisconnectedUnspecified => Self::TryRecvDisconnectedError(id),
            any => any
        }
    }
}
/*
impl Display for CommError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}*/

impl<Spec: DomainParameters> From<RecvError> for CommError<Spec>{
    fn from(_: RecvError) -> Self {
        Self::RecvErrorUnspecified
    }
}
impl<Spec: DomainParameters, T> From<SendError<T>> for CommError<Spec>{
    fn from(_: SendError<T>) -> Self {
        Self::SendErrorUnspecified
    }
}
impl<Spec: DomainParameters> From<TryRecvError> for CommError<Spec>{
    fn from(e: TryRecvError) -> Self {
        match e{
            TryRecvError::Empty => Self::TryRecvErrorEmptyUnspecified,
            TryRecvError::Disconnected => Self::TryRecvErrorDisconnectedUnspecified
        }
    }
}
impl<Spec: DomainParameters, T> From<TrySendError<T>> for CommError<Spec>{
    fn from(_: TrySendError<T>) -> Self {
        Self::TrySendErrorUnspecified
    }
}

impl <Spec: DomainParameters> From<CommError<Spec>> for SztormError<Spec>{
    fn from(value: CommError<Spec>) -> Self {
        Self::Comm(value)
    }
}