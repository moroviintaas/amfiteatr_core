
use crate::error::SztormError;
use crate::protocol::DomainParameters;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SetupError<Spec: DomainParameters>{
    #[error("Agent's Id: {0} is duplicated")]
    DuplicateId(Spec::AgentId),
    #[error("Missing Agent's Id: {0}")]
    MissingId(Spec::AgentId),
    #[error("Missing environment initial state")]
    MissingState,
    #[error("Missing action processing function")]
    MissingActionProcessingFunction


}
impl<Spec: DomainParameters> From<SetupError<Spec>> for SztormError<Spec>{
    fn from(value: SetupError<Spec>) -> Self {
        SztormError::Setup(value)
    }
}