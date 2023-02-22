use std::error::Error;
use std::fmt::Debug;

pub trait CommunicatingAgent{
    type Outward;
    type Inward;
    type CommunicationError: Error;

    fn send(&mut self, message: Self::Outward) -> Result<(), Self::CommunicationError>;
    fn recv(&mut self) -> Result<Self::Inward, Self::CommunicationError>;
}