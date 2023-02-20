use std::error::Error;
use std::fmt::Debug;

pub trait CommEndpoint{
    type OutwardType;
    type InwardType;
    type Error: Debug + Error;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error>;
    fn recv(&mut self) -> Result<Self::InwardType, Self::Error>;
    fn try_recv(&mut self) -> Result<Self::InwardType, Self::Error>;
}

impl<T> CommEndpoint for Box<T>
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