use crate::action::Action;
use crate::{InformationSet, Policy, StatefulAgent};

pub trait ActingAgent {
    type Act: Action;

    fn take_action(&self) -> Option<Self::Act>;
}

pub trait PolicyAgent: StatefulAgent{
    type Policy: Policy<StateType = <Self as StatefulAgent>::State>;

    fn policy(&self) -> &Self::Policy;
    fn policy_select_action(&self)
        -> Option<<<Self as StatefulAgent>::State as InformationSet>::ActionType>{
        self.policy().select_action(self.state())
    }
}