use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::error::{CommError, ProtocolError};
use crate::protocol::{ProtocolSpecification};

#[derive(Debug, Clone,  Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum TurError<Spec: ProtocolSpecification>{
    GameError(Spec::GameErrorType),
    CommError(CommError),
    ProtocolError(ProtocolError<Spec>),
}

impl <Spec: ProtocolSpecification> Display for TurError<Spec>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            TurError::GameError(e) => write!(f, "GameError: {e}"),
            TurError::CommError(e) => write!(f, "CommError: {e}"),
            TurError::ProtocolError(e) => write!(f, "ProtocolError: {e}"),

        }

    }
}


/*
impl<Spec: ProtocolSpecification> From<Spec::GameErrorType> for TurError<Spec>{
    fn from(value: Spec::GameErrorType) -> Self {
        Self::GameError(value)
    }
}*/