use std::fmt::{Display, Formatter, write};
use thiserror::Error;
use crate::error::{CommError, ProtocolError, SetupError};
use crate::protocol::{ProtocolSpecification};

#[derive(Debug, Clone,  Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SztormError<Spec: ProtocolSpecification>{
    Game(Spec::GameErrorType),
    Comm(CommError<Spec>),
    Protocol(ProtocolError<Spec>),
    Setup(SetupError<Spec>)
}

impl <Spec: ProtocolSpecification> Display for SztormError<Spec>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            SztormError::Game(e) => write!(f, "GameError: {e}"),
            SztormError::Comm(e) => write!(f, "CommError: {e}"),
            SztormError::Protocol(e) => write!(f, "ProtocolError: {e}"),
            SztormError::Setup(e) => write!(f, "SetupError: {e}")

        }

    }
}


/*
impl<Spec: ProtocolSpecification> From<Spec::GameErrorType> for TurError<Spec>{
    fn from(value: Spec::GameErrorType) -> Self {
        Self::GameError(value)
    }
}*/