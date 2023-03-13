use std::sync::mpsc::{RecvError, SendError, TryRecvError, TrySendError};
use thiserror::Error;
use crate::error::TurError;
use crate::protocol::ProtocolSpecification;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum CommError{
    #[error("Send Error")]
    SendError,
    #[error("TrySend Error")]
    TrySendError,
    #[error("RecvSend Error")]
    RecvError,
    #[error("TryRecv Error (empty)")]
    TryRecvEmptyError,
    #[error("TryRecv Error (disconnected)")]
    TryRecvDisconnectedError,
    #[error("Serialize Error")]
    SerializeError,
    #[error("Deserialize Error")]
    DeserializeError,
}
/*
impl Display for CommError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}*/

impl From<RecvError> for CommError{
    fn from(_: RecvError) -> Self {
        Self::RecvError
    }
}
impl<T> From<SendError<T>> for CommError{
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}
impl From<TryRecvError> for CommError{
    fn from(e: TryRecvError) -> Self {
        match e{
            TryRecvError::Empty => Self::TryRecvEmptyError,
            TryRecvError::Disconnected => Self::TryRecvDisconnectedError
        }
    }
}
impl<T> From<TrySendError<T>> for CommError{
    fn from(_: TrySendError<T>) -> Self {
        Self::TrySendError
    }
}

impl <Spec: ProtocolSpecification> From<CommError> for TurError<Spec>{
    fn from(value: CommError) -> Self {
        Self::CommError(value)
    }
}