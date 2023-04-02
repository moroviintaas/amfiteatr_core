use crate::agent::AgentActionPair;
use crate::error::SztormError;
use crate::protocol::v1::spec::ProtocolSpecification;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum EnvMessage<Spec: ProtocolSpecification>{
    YourMove,
    GameFinished,
    Kill,
    UpdateState(Spec::UpdateType),
    ActionNotify(AgentActionPair<Spec::AgentId, Spec::ActionType>),
    ErrorNotify(SztormError<Spec>)
}