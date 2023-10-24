mod round_robin_model;

pub use round_robin_model::rr;
use crate::error::AmfiError;
use crate::domain::DomainParameters;

pub trait AutomaticAgent{
    type ProtocolSpecType: DomainParameters;

    fn run(&mut self) -> Result<(), AmfiError<Self::ProtocolSpecType>>;
}
