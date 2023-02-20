use crate::agent::AgentActionPair;
use crate::error::TurError;
use crate::protocol::v1::spec::ProtocolSpecification;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum EnvMessage<Spec: ProtocolSpecification>{
    YourMove,
    GameFinished,
    Kill,
    UpdateState(Spec::UpdateType),
    ActionNotify(AgentActionPair<Spec::AgentType, Spec::ActionType>),
    ErrorNotify(TurError<Spec>)
}