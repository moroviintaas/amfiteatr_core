mod state_update;

pub use state_update::*;
use crate::protocol::DomainParameters;

pub trait ConstructedState<DP: DomainParameters, B>{

    fn construct_from(base: B) -> Self;

    fn construct_similar_from(&mut self, base: B) -> Self where Self: Sized{
        Self::construct_from(base)
    }

}

impl<DP: DomainParameters, B, T: ConstructedState<DP, B>> ConstructedState<DP, B> for Box<T>{

    fn construct_from(base: B) -> Self {
        Box::new(T::construct_from(base))
    }


}