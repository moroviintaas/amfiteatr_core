use crate::agent::AgentActionPair;
use crate::error::SztormError;
use crate::domain::v1::domain_parameters::DomainParameters;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum EnvMessage<Spec: DomainParameters>{
    YourMove,
    MoveRefused,
    GameFinished,
    GameFinishedWithIllegalAction(Spec::AgentId),
    Kill,
    UpdateState(Spec::UpdateType),
    ActionNotify(AgentActionPair<Spec::AgentId, Spec::ActionType>),
    RewardFragment(Spec::UniversalReward),
    ErrorNotify(SztormError<Spec>),

}