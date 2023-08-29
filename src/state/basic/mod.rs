mod state_update;

pub use state_update::*;
use crate::protocol::DomainParameters;

pub trait ConstructedState<DP: DomainParameters, B>{

    fn construct_from(base: B) -> Self;
}

impl<DP: DomainParameters, B, T: ConstructedState<DP, B>> ConstructedState<DP, B> for Box<T>{

    fn construct_from(base: B) -> Self {
        Box::new(T::construct_from(base))
    }
}