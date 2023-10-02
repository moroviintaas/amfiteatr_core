mod round_robin_model;

pub use round_robin_model::rr;
use crate::error::SztormError;
use crate::protocol::DomainParameters;

pub trait AutomaticAgent{
    type ProtocolSpecType: DomainParameters;

    fn run(&mut self) -> Result<(), SztormError<Self::ProtocolSpecType>>;
}
