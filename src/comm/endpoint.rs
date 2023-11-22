use std::error::Error;
use std::fmt::Debug;

use crate::{domain::{EnvMessage, DomainParameters, AgentMessage}, error::CommunicationError};
/// Trait for structures using to communicate in synchronous mode between two objects.
pub trait CommPort {
    /// The type that is sent via this endpoint.
    /// In scope of this crate, for environment it will be usually
    /// [`EnvMessage`](crate::domain::EnvMessage) or [`AgentMessage`](crate::domain::AgentMessage)
    type OutwardType: Debug;
    /// The type that is received via this endpoint.
    /// In scope of this crate, for environment it will be usually
    /// [`EnvMessage`](crate::domain::EnvMessage) or [`AgentMessage`](crate::domain::AgentMessage)
    type InwardType: Debug;
    /// The error type that can be caused during communication.
    /// In scope of this crate, for environment it will be usually
    /// [`CommunicationError`](crate::error::CommunicationError)
    type Error: Debug + Error;

    /// Method used to send message. Message can be queued on the side of receiver.
    /// Sender should not block waiting for receiver to consume message.
    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error>;
    /// Method used to receive message. This method should block waiting for message to come.
    fn receive_blocking(&mut self) -> Result<Self::InwardType, Self::Error>;
    /// Method used to receive message. This method should not block.
    fn receive_non_blocking(&mut self) -> Result<Option<Self::InwardType>, Self::Error>;
}

impl<T: ?Sized> CommPort for Box<T>
where T: CommPort {
    type OutwardType = T::OutwardType;
    type InwardType = T::InwardType;

    type Error = T::Error;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error> {
        self.as_mut().send(message)
    }

    fn receive_blocking(&mut self) -> Result<Self::InwardType, Self::Error> {
        self.as_mut().receive_blocking()
    }

    fn receive_non_blocking(&mut self) -> Result<Option<Self::InwardType>, Self::Error> {
        self.as_mut().receive_non_blocking()
    }
}

pub trait EnvCommEndpoint<Spec: DomainParameters>: CommPort<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommunicationError<Spec>>{}

impl<Spec: DomainParameters, T> EnvCommEndpoint<Spec> for T
where T: CommPort<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommunicationError<Spec>>{}

pub trait AgentCommEndpoint<Spec: DomainParameters>: CommPort<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommunicationError<Spec>>{}

impl<Spec: DomainParameters, T> AgentCommEndpoint<Spec> for T
where T: CommPort<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommunicationError<Spec>>{}


pub trait EnvironmentAdapter<DP: DomainParameters>{

    fn send(&mut self, agent: &DP::AgentId, message: EnvMessage<DP>) 
    -> Result<(), CommunicationError<DP>>;

    fn receive_blocking(&mut self) -> Result<(DP::AgentId, AgentMessage<DP>), CommunicationError<DP>>;
    fn receive_non_blocking(&mut self) -> Result<Option<(DP::AgentId, AgentMessage<DP>)>, CommunicationError<DP>>;

    fn is_agent_connected(&self, agent_id: &DP::AgentId) -> bool;
}

pub trait AgentAdapter<DP: DomainParameters>{
    fn send(&mut self, message: AgentMessage<DP>) -> Result<(), CommunicationError<DP>>;
    fn receive(&mut self) -> Result<EnvMessage<DP>, CommunicationError<DP>>;
}

pub trait BroadcastingEnvironmentAdapter<DP: DomainParameters>: EnvironmentAdapter<DP>{
    fn send_all(&mut self, message: EnvMessage<DP>) ->  Result<(), CommunicationError<DP>>;
}