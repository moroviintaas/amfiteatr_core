use std::marker::PhantomData;
use crate::protocol::ProtocolSpecification;
use crate::{InformationSet, Policy, QFunction};

pub struct SingleQPolicyGen<
    Spec: ProtocolSpecification,
    QFunc: QFunction<Spec>>{
    q_function: QFunc,
    _spec: PhantomData<Spec>


}

impl<
    Spec: ProtocolSpecification,
    QFunc: QFunction<Spec>> Policy<Spec> for SingleQPolicyGen<Spec,  QFunc> {
    type StateType = QFunc::StateType;

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