mod state_update;

pub use state_update::*;
use crate::protocol::DomainParameters;

pub trait ConstructedState<DP: DomainParameters, B>{

    fn from_base_ref(base: &B) -> Self;
    fn from_base_consume(base: B) -> Self;
}

impl<DP: DomainParameters, B, T: ConstructedState<DP, B>> ConstructedState<DP, B> for Box<T>{
    fn from_base_ref(base: &B) -> Self {
        Box::new(T::from_base_ref(base))
    }

    fn from_base_consume(base: B) -> Self {
        Box::new(T::from_base_consume(base))
    }
}