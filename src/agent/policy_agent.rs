
use crate::{Policy, StatefulAgent};
use crate::protocol::ProtocolSpecification;

pub trait ActingAgent<Spec: ProtocolSpecification> {

    fn take_action(&self) -> Option<Spec::ActionType>;
}

pub trait PolicyAgent<Spec: ProtocolSpecification>: StatefulAgent<Spec>{
    type Policy: Policy<Spec, StateType = <Self as StatefulAgent<Spec>>::State>;

    fn policy(&self) -> &Self::Policy;
    fn policy_select_action(&self)
        -> Option<Spec::ActionType>{
        self.policy().select_action(self.state())
    }
}