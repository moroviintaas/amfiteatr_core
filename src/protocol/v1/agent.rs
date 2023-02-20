
use crate::error::TurError;
use crate::protocol::v1::spec::ProtocolSpecification;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum AgentMessage<Spec: ProtocolSpecification>{
    PerformAction(Spec::ActionType),
    NotifyError(TurError<Spec>),
    Quit,

}