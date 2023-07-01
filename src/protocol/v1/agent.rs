
use crate::error::SztormError;
use crate::protocol::v1::domain_parameters::DomainParameters;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum AgentMessage<Spec: DomainParameters>{
    TakeAction(Spec::ActionType),
    NotifyError(SztormError<Spec>),
    Quit,

}