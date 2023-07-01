use crate::error::SztormError;
use crate::protocol::DomainParameters;

pub trait AutomaticEnvironment<Spec: DomainParameters>{
    fn run(&mut self) -> Result<(), SztormError<Spec>>;
}

