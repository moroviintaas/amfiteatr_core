use crate::protocol::DomainParameters;
use crate::{InformationSet};

pub trait QFunction<Spec: DomainParameters>{

    type StateType: InformationSet<Spec>;
    type QValue: PartialOrd;

    fn q_value(&self, state: &Self::StateType, action: &<<Self::StateType as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::Item) -> Result<Self::QValue, Spec::GameErrorType>;

    fn q_value_unchecked(&self, state: &Self::StateType, action: & <<Self::StateType as InformationSet<Spec>>::ActionIteratorType as IntoIterator>::Item) -> Self::QValue{
        self.q_value(state, action ).unwrap()
    }
}