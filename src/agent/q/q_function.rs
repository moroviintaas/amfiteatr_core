use crate::protocol::ProtocolSpecification;
use crate::{InformationSet};

pub trait QFunction<Spec: ProtocolSpecification>{

    type StateType: InformationSet<Spec>;
    type QValue: Ord;

    fn q_value(&self, state: &Self::StateType, action: &<<Self::StateType as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::Item) -> Result<Self::QValue, Spec::GameErrorType>;

    fn q_value_unchecked(&self, state: &Self::StateType, action: & <<Self::StateType as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::Item) -> Self::QValue{
        self.q_value(state, action ).unwrap()
    }
}