use std::error::Error;
use std::fmt::Debug;

use crate::{domain::{EnvMessage, DomainParameters, AgentMessage}, error::CommError};
/// Trait for structures using to communicate in synchronous mode between two objects.
pub trait CommEndpoint{
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
    /// [`CommError`](crate::error::CommError)
    type Error: Debug + Error;

    /// Method used to send message. Message can be queued on the side of receiver.
    /// Sender should not block waiting for receiver to consume message.
    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error>;
    /// Method used to receive message. This method should block waiting for message to come.
    fn receive_blocking(&mut self) -> Result<Self::InwardType, Self::Error>;
    /// Method used to receive message. This method should not block.
    fn receive_non_blocking(&mut self) -> Result<Option<Self::InwardType>, Self::Error>;
}

impl<T: ?Sized> CommEndpoint for Box<T>
where T: CommEndpoint{
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

pub trait EnvCommEndpoint<Spec: DomainParameters>: CommEndpoint<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommError<Spec>>{}

impl<Spec: DomainParameters, T> EnvCommEndpoint<Spec> for T
where T: CommEndpoint<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommError<Spec>>{}

pub trait AgentCommEndpoint<Spec: DomainParameters>: CommEndpoint<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommError<Spec>>{}

impl<Spec: DomainParameters, T> AgentCommEndpoint<Spec> for T
where T: CommEndpoint<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommError<Spec>>{}
