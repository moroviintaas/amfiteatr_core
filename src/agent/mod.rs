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
use crate::comm::BidirectionalEndpoint;
use crate::domain::{AgentMessage, DomainParameters, EnvironmentMessage};
use crate::error::CommunicationError;

/*
pub trait AgentCommPort<DP: DomainParameters>
    : BidirectionalEndpoint<OutwardType = AgentMessage<DP>, InwardType=EnvironmentMessage<DP>, Error = CommunicationError<DP>>{}

impl<DP: DomainParameters, T: BidirectionalEndpoint<OutwardType = AgentMessage<DP>, InwardType=EnvironmentMessage<DP>, Error = CommunicationError<DP>>>
AgentCommPort<DP> for T{}

 */