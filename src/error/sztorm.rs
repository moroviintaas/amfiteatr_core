use thiserror::Error;
use crate::error::{CommError, ProtocolError, SetupError};
use crate::protocol::{DomainParameters};

#[derive(Debug, Clone,  Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SztormError<Spec: DomainParameters>{
    #[error("Game error: {0}")]
    Game(Spec::GameErrorType),
    #[error("Agent {1} caused game error: {0}")]
    GameWithConvict(Spec::GameErrorType, Spec::AgentId),
    #[error("Communication error: {0}")]
    Comm(CommError<Spec>),
    #[error("Protocol error: {0}")]
    Protocol(ProtocolError<Spec>),
    #[error("Setup error: {0}")]
    Setup(SetupError<Spec>),
    #[error("Data convert")]
    DataConvert()
}
