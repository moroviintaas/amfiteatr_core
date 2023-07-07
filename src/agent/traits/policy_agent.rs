use crate::agent::{Policy, StatefulAgent};
use crate::protocol::DomainParameters;

pub trait ActingAgent<Spec: DomainParameters> {

    fn take_action(&mut self) -> Option<Spec::ActionType>;
    fn finalize(&mut self);
}

pub trait PolicyAgent<Spec: DomainParameters>: StatefulAgent<Spec>{
    type Policy: Policy<Spec, StateType = <Self as StatefulAgent<Spec>>::State>;

    fn policy(&self) -> &Self::Policy;
    fn policy_mut(&mut self) -> &mut Self::Policy;
    fn policy_select_action(&self)
        -> Option<Spec::ActionType>{
        self.policy().select_action(self.state())
    }
}