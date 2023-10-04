use std::error::Error;
use crate::domain::DomainParameters;

pub trait InternalGameError<Spec: DomainParameters>: Error + Clone{

}


impl<T: Error + Clone, DP:DomainParameters> InternalGameError<DP> for T{

}
/*
impl<Internal, Spec: ProtocolSpecification> From<Internal> for TurError<Spec>{
    fn from(value: Internal) -> Self {
        Self::GameError(value)
    }
}*/