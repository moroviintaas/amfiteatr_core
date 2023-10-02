use thiserror::Error;
use crate::error::SztormError;
use crate::protocol::DomainParameters;

#[derive(Debug, Clone, Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum WorldError<DP: DomainParameters>{

    #[error("Failed joining thread for agent: {0}")]
    FailedJoinAgent(DP::AgentId)
}

impl<DP: DomainParameters> From<WorldError<DP>> for SztormError<DP>{
    fn from(value: WorldError<DP>) -> Self {
        Self::Internal(value)
    }
}