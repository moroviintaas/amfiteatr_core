use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::error::{CommError, ProtocolError};
use crate::protocol::{ProtocolSpecification};

#[derive(Debug, Clone,  Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SztormError<Spec: ProtocolSpecification>{
    GameError(Spec::GameErrorType),
    CommError(CommError),
    ProtocolError(ProtocolError<Spec>),
}

impl <Spec: ProtocolSpecification> Display for SztormError<Spec>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            SztormError::GameError(e) => write!(f, "GameError: {e}"),
            SztormError::CommError(e) => write!(f, "CommError: {e}"),
            SztormError::ProtocolError(e) => write!(f, "ProtocolError: {e}"),

        }

    }
}


/*
impl<Spec: ProtocolSpecification> From<Spec::GameErrorType> for TurError<Spec>{
    fn from(value: Spec::GameErrorType) -> Self {
        Self::GameError(value)
    }
}*/