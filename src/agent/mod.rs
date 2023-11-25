mod identifier;
mod action_pair;
mod policy;
mod traits;
mod trajectory;
mod generic;
mod info_set;

pub use identifier::*;
pub use action_pair::*;
pub use policy::*;
pub use traits::*;
pub use trajectory::*;
pub use generic::*;
pub use info_set::*;
use crate::comm::CommPort;
use crate::domain::{AgentMessage, DomainParameters, EnvMessage};
use crate::error::CommunicationError;

pub trait AgentCommPort<DP: DomainParameters>
    :CommPort<OutwardType = AgentMessage<DP>, InwardType=EnvMessage<DP>, Error = CommunicationError<DP>>{}

impl<DP: DomainParameters, T: CommPort<OutwardType = AgentMessage<DP>, InwardType=EnvMessage<DP>, Error = CommunicationError<DP>>>
AgentCommPort<DP> for T{}