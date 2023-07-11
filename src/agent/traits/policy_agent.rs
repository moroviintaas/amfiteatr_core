use crate::agent::{Agent, Policy, StatefulAgent};
use crate::protocol::DomainParameters;

pub trait ActingAgent<DP: DomainParameters>: Agent<DP> {

    fn take_action(&mut self) -> Option<DP::ActionType>;
    fn finalize(&mut self);
}

pub trait PolicyAgent<DP: DomainParameters>: StatefulAgent<DP>{
    type Policy: Policy<DP, StateType = <Self as StatefulAgent<DP>>::State>;

    fn policy(&self) -> &Self::Policy;
    fn policy_mut(&mut self) -> &mut Self::Policy;
    fn policy_select_action(&self)
        -> Option<DP::ActionType>{
        self.policy().select_action(self.state())
    }
}