use std::marker::PhantomData;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent, RewardedAgent, Agent, ResetAgent};
use crate::agent::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::{Reward};
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};
use crate::state::State;

pub struct AgentGen<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>{
    state: <P as Policy<DP>>::StateType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<DP>,

    id: DP::AgentId,
    constructed_universal_reward: <DP as DomainParameters>::UniversalReward,
    actual_universal_score: <DP as DomainParameters>::UniversalReward,
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
    AgentGen<DP, P, Comm>{

    pub fn new(id: DP::AgentId, state: <P as Policy<DP>>::StateType, comm: Comm, policy: P) -> Self{
        Self{state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            actual_universal_score: Reward::neutral() }
    }

    pub fn replace_state(&mut self, state: <P as Policy<DP>>::StateType){
        self.state = state
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
    CommunicatingAgent<DP> for AgentGen<DP, P, Comm>
{
    type CommunicationError = CommError<DP>;

    fn send(&mut self, message: AgentMessage<DP>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<DP>, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
StatefulAgent<DP> for AgentGen<DP, P, Comm>{
    type State = <P as Policy<DP>>::StateType;

    fn update(&mut self, state_update: DP::UpdateType) -> Result<(), DP::GameErrorType> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
ActingAgent<DP> for AgentGen<DP, P, Comm>{

    fn take_action(&mut self) -> Option<DP::ActionType> {
        self.policy.select_action(&self.state)

    }

    fn finalize(&mut self) {

    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
PolicyAgent<DP> for AgentGen<DP, P, Comm>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }

    fn policy_mut(&mut self) -> &mut Self::Policy {
        &mut self.policy
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
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

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
ResetAgent<DP> for AgentGen<DP, P, Comm>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::State) {
        self.state = initial_state;
        self.constructed_universal_reward = DP::UniversalReward::neutral();
        self.actual_universal_score = DP::UniversalReward::neutral();
    }
}