use std::fmt::{Display, Formatter};
use crate::error::SztormError;
use crate::protocol::ProtocolSpecification;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SetupError<Spec: ProtocolSpecification>{
    #[error("Agent's Id: {0} is duplicated")]
    DuplicateId(Spec::AgentId),
    #[error("Missing Agent's Id: {0}")]
    MissingId(Spec::AgentId),
    #[error("Missing environment initial state")]
    MissingState,
    #[error("Missing action processing function")]
    MissingActionProcessingFunction


}
impl<Spec: ProtocolSpecification> From<SetupError<Spec>> for SztormError<Spec>{
    fn from(value: SetupError<Spec>) -> Self {
        SztormError::Setup(value)
    }
}