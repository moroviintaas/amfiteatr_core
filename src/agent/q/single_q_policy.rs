use std::marker::PhantomData;
use crate::agent::{Policy, QFunction};
use crate::domain::DomainParameters;
use crate::state::agent::InformationSet;

pub struct SingleQPolicyGen<
    Spec: DomainParameters,
    QFunc: QFunction<Spec>>{
    q_function: QFunc,
    _spec: PhantomData<Spec>

}

impl<
    Spec: DomainParameters,
    QFunc: QFunction<Spec>> SingleQPolicyGen<Spec, QFunc>{

    pub fn new(q_function: QFunc) -> Self{
        Self{q_function, _spec: PhantomData::default()}
    }

    pub fn q_function(&self) -> &QFunc{
        &self.q_function
    }

    pub fn q_function_mut(&mut self) -> &mut QFunc{
        &mut self.q_function
    }
}

impl<
    Spec: DomainParameters,
    QFunc: QFunction<Spec>> Policy<Spec> for SingleQPolicyGen<Spec,  QFunc> {
    type InfoSetType = QFunc::StateType;

    fn select_action(&self, state: &QFunc::StateType) -> Option<Spec::ActionType> {
        state.available_actions().into_iter()
            .fold(None, |current_best, action|{
                let q = self.q_function.q_value_unchecked(state, &action);
                if let Some((best,q_best)) = current_best{
                    if q > q_best{
                        Some((action, q))
                    } else {
                        Some((best, q_best))
                    }
                } else{
                    Some((action, q))
                }

            }).map(|(action, _q)| action)




    }
}