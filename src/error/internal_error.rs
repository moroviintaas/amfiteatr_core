use std::error::Error;
use crate::error::SztormError;
use crate::domain::DomainParameters;

pub trait InternalGameError<Spec: DomainParameters>: Error + Into<SztormError<Spec>> + Clone{

}
/*
impl<Internal, Spec: ProtocolSpecification> From<Internal> for TurError<Spec>{
    fn from(value: Internal) -> Self {
        Self::GameError(value)
    }
}*/