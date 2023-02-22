use std::error::Error;
use thiserror::Error;
use crate::error::{CommError, ProtocolError};
use crate::protocol::{ProtocolSpecification};

#[derive(Debug, Clone,  Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum TurError<Spec: ProtocolSpecification>{
    GameError(Spec::GameErrorType),
    CommError(CommError),
    ProtocolError(ProtocolError<Spec>)
}