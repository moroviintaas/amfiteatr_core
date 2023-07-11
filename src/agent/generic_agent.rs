use std::marker::PhantomData;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent, RewardedAgent, Agent};
use crate::agent::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::{Reward};
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};
use crate::state::State;

pub struct AgentGen<Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>{
    state: <P as Policy<Spec>>::StateType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<Spec>,

    id: Spec::AgentId,
    constructed_universal_reward: <Spec as DomainParameters>::UniversalReward,
    actual_universal_score: <Spec as DomainParameters>::UniversalReward,
}

impl <Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    AgentGen<Spec, P, Comm>{

    pub fn new(id: Spec::AgentId, state: <P as Policy<Spec>>::StateType, comm: Comm, policy: P) -> Self{
        Self{state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            actual_universal_score: Reward::neutral() }
    }

    pub fn replace_state(&mut self, state: <P as Policy<Spec>>::StateType){
        self.state = state
    }
}

impl<Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    CommunicatingAgent<Spec> for AgentGen<Spec, P, Comm>
{
    //type Outward = AgentMessage<Spec>;
    //type Inward = EnvMessage<Spec>;
    type CommunicationError = CommError<Spec>;

    fn send(&mut self, message: AgentMessage<Spec>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<Spec>, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    StatefulAgent<Spec> for AgentGen<Spec, P, Comm>{
    type State = <P as Policy<Spec>>::StateType;

    fn update(&mut self, state_update: Spec::UpdateType) -> Result<(), Spec::GameErrorType> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
ActingAgent<Spec> for AgentGen<Spec, P, Comm>{

    fn take_action(&mut self) -> Option<Spec::ActionType> {
        self.policy.select_action_mut(&self.state)
        //self.policy_select_action()
    }

    fn finalize(&mut self) {

    }
}

impl<Spec: DomainParameters, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
PolicyAgent<Spec> for AgentGen<Spec, P, Comm>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }

    fn policy_mut(&mut self) -> &mut Self::Policy {
        &mut self.policy
    }
}

impl<DP: DomainParameters, P: Policy<DP>,
    Comm: CommEndpoint<OutwardType=AgentMessage<DP>, InwardType=EnvMessage<DP>, Error=CommError<DP>>>
Agent<DP> for AgentGen<DP, P, Comm>{

    fn id(&self) -> DP::AgentId {
        self.id
    }
}

impl<Spec: DomainParameters,
    P: Policy<Spec>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<Spec>,
        InwardType=EnvMessage<Spec>,
        Error=CommError<Spec>>> RewardedAgent<Spec> for AgentGen<Spec, P, Comm>{
    fn current_universal_reward(&self) -> Spec::UniversalReward {
        self.constructed_universal_reward.clone()
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &Spec::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> Spec::UniversalReward {
        self.actual_universal_score.clone() + &self.constructed_universal_reward
    }
}