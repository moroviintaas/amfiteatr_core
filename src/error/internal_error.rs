use std::error::Error;
use crate::error::TurError;
use crate::protocol::ProtocolSpecification;

pub trait InternalGameError<Spec: ProtocolSpecification>: Error + Into<TurError<Spec>> + Clone{

}
/*
impl<Internal, Spec: ProtocolSpecification> From<Internal> for TurError<Spec>{
    fn from(value: Internal) -> Self {
        Self::GameError(value)
    }
}*/