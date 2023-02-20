use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError, TryRecvError, TrySendError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum CommError{

    SendError,
    TrySendError,
    RecvError,
    TryRecvError
}
impl Display for CommError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

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
    fn from(_: TryRecvError) -> Self {
        Self::TryRecvError
    }
}
impl<T> From<TrySendError<T>> for CommError{
    fn from(_: TrySendError<T>) -> Self {
        Self::TrySendError
    }
}