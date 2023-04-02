
use crate::error::SztormError;
use crate::protocol::v1::spec::ProtocolSpecification;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum AgentMessage<Spec: ProtocolSpecification>{
    TakeAction(Spec::ActionType),
    NotifyError(SztormError<Spec>),
    Quit,

}