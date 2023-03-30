mod round_robin_model;
pub use round_robin_model::rr;
use crate::error::TurError;
use crate::protocol::ProtocolSpecification;

pub trait AutomaticAgent{
    type ProtocolSpecType: ProtocolSpecification;

    fn run(&mut self) -> Result<(), TurError<Self::ProtocolSpecType>>;
}
