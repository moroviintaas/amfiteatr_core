use std::error::Error;
use std::fmt::Debug;

use crate::{protocol::{EnvMessage, DomainParameters, AgentMessage}, error::CommError};

pub trait CommEndpoint{
    type OutwardType: Debug;
    type InwardType: Debug;
    type Error: Debug + Error;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error>;
    fn recv(&mut self) -> Result<Self::InwardType, Self::Error>;
    fn try_recv(&mut self) -> Result<Self::InwardType, Self::Error>;
}

impl<T: ?Sized> CommEndpoint for Box<T>
where T: CommEndpoint{
    type OutwardType = T::OutwardType;
    type InwardType = T::InwardType;

    type Error = T::Error;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error> {
        self.as_mut().send(message)
    }

    fn recv(&mut self) -> Result<Self::InwardType, Self::Error> {
        self.as_mut().recv()
    }

    fn try_recv(&mut self) -> Result<Self::InwardType, Self::Error> {
        self.as_mut().try_recv()
    }
}

pub trait EnvCommEndpoint<Spec: DomainParameters>: CommEndpoint<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommError<Spec>>{}

impl<Spec: DomainParameters, T> EnvCommEndpoint<Spec> for T
where T: CommEndpoint<OutwardType = EnvMessage<Spec>, InwardType = AgentMessage<Spec>, Error = CommError<Spec>>{}

pub trait AgentCommEndpoint<Spec: DomainParameters>: CommEndpoint<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommError<Spec>>{}

impl<Spec: DomainParameters, T> AgentCommEndpoint<Spec> for T
where T: CommEndpoint<OutwardType = AgentMessage<Spec>, InwardType = EnvMessage<Spec>, Error = CommError<Spec>>{}
