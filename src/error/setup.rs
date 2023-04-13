use std::fmt::{Display, Formatter};
use crate::protocol::ProtocolSpecification;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum SetupError<Spec: ProtocolSpecification>{
    DuplicateId(Spec::AgentId)
}

impl<Spec: ProtocolSpecification> Display for SetupError<Spec>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            SetupError::DuplicateId(id) => write!(f, "Duplicated Id: {id:}")
        }
    }
}