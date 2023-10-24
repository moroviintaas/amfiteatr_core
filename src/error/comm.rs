use std::sync::mpsc::{RecvError, SendError, TryRecvError, TrySendError};
use thiserror::Error;

use crate::error::AmfiError;
use crate::domain::DomainParameters;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum CommError<DP: DomainParameters>{
    #[error("Send Error to {0}, text: {1}")]
    SendError(DP::AgentId, String),
    #[error("Send Error, text: {0}")]
    SendErrorUnspecified(String),
    #[error("Broadcast Send Error (on {0})")]
    BroadcastSendError(DP::AgentId),
    #[error("Broadcast Send Error")]
    BroadcastSendErrorUnspecified,
    #[error("Recv Error from {0}, text: {1}")]
    RecvError(DP::AgentId, String),
    #[error("Recv Error, text: {0}")]
    RecvErrorUnspecified(String),
    #[error("TryRecv Error (empty) from {0}")]
    RecvEmptyBufferError(DP::AgentId),
    #[error("TryRecv Error (empty")]
    RecvEmptyBufferErrorUnspecified,
    #[error("TryRecv Error (disconnected")]
    RecvPeerDisconnectedErrorUnspecified,
    #[error("TryRecv Error (disconnected) from {0}")]
    RecvPeerDisconnectedError(DP::AgentId),
    #[error("Serialize Error, text: {0}")]
    SerializeError(String),
    #[error("Deserialize Error, text: {0}")]
    DeserializeError(String),
    #[error("No such connection")]
    NoSuchConnection,

}

impl<Spec: DomainParameters> CommError<Spec>{

    pub fn specify_id(self, id: Spec::AgentId) -> Self{
        match self{
            CommError::SendErrorUnspecified(s) => Self::SendError(id, s),
            CommError::BroadcastSendErrorUnspecified => Self::BroadcastSendError(id),
            CommError::RecvErrorUnspecified(s) => Self::RecvError(id,s),
            CommError::RecvEmptyBufferErrorUnspecified => Self::RecvEmptyBufferError(id),
            CommError::RecvPeerDisconnectedErrorUnspecified => Self::RecvPeerDisconnectedError(id),
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
    fn from(e: RecvError) -> Self {
        Self::RecvErrorUnspecified(format!("{e:}"))
    }
}
impl<Spec: DomainParameters, T> From<SendError<T>> for CommError<Spec>{
    fn from(e: SendError<T>) -> Self {
        Self::SendErrorUnspecified(format!("{e:}"))
    }
}
impl<Spec: DomainParameters> From<TryRecvError> for CommError<Spec>{
    fn from(e: TryRecvError) -> Self {
        match e{
            TryRecvError::Empty => Self::RecvEmptyBufferErrorUnspecified,
            TryRecvError::Disconnected => Self::RecvPeerDisconnectedErrorUnspecified
        }
    }
}

impl<Spec: DomainParameters, T> From<TrySendError<T>> for CommError<Spec>{
    fn from(e: TrySendError<T>) -> Self {
        Self::SendErrorUnspecified(format!("{e:}"))
    }
}

impl <Spec: DomainParameters> From<CommError<Spec>> for AmfiError<Spec>{
    fn from(value: CommError<Spec>) -> Self {
        Self::Comm(value)
    }
}