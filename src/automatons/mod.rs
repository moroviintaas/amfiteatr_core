mod round_robin_model;
pub use round_robin_model::rr;
use crate::error::SztormError;
use crate::protocol::ProtocolSpecification;

pub trait AutomaticAgent{
    type ProtocolSpecType: ProtocolSpecification;

    fn run(&mut self) -> Result<(), SztormError<Self::ProtocolSpecType>>;
}
