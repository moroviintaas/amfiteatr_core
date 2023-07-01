use crate::agent::AgentActionPair;
use crate::error::SztormError;
use crate::protocol::v1::domain_parameters::DomainParameters;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum EnvMessage<Spec: DomainParameters>{
    YourMove,
    GameFinished,
    Kill,
    UpdateState(Spec::UpdateType),
    ActionNotify(AgentActionPair<Spec::AgentId, Spec::ActionType>),
    ErrorNotify(SztormError<Spec>)
}