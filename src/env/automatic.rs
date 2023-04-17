use crate::error::SztormError;
use crate::protocol::ProtocolSpecification;

pub trait AutomaticEnvironment<Spec: ProtocolSpecification>{
    fn run(&mut self) -> Result<(), SztormError<Spec>>;
}

