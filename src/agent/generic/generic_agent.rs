use std::marker::PhantomData;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent, EnvRewardedAgent, Agent, ResetAgent, InternalRewardedAgent, AgentGenT};
use crate::agent::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::{Reward};
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};
use crate::state::agent::{InformationSet, ScoringInformationSet};

pub struct AgentGen<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
    state: <P as Policy<DP>>::StateType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<DP>,

    id: DP::AgentId,
    constructed_universal_reward: <DP as DomainParameters>::UniversalReward,
    actual_universal_score: <DP as DomainParameters>::UniversalReward,
    explicit_subjective_reward_component: <P::StateType as ScoringInformationSet<DP>>::RewardType,
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
    AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    pub fn new(id: DP::AgentId, state: <P as Policy<DP>>::StateType, comm: Comm, policy: P) -> Self{
        Self{state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            actual_universal_score: Reward::neutral(),
            explicit_subjective_reward_component: <P::StateType as ScoringInformationSet<DP>>::RewardType::neutral()
        }
    }

    pub fn replace_state(&mut self, state: <P as Policy<DP>>::StateType){
        self.state = state
    }

    pub fn do_change_policy<P2: Policy<DP, StateType=P::StateType>>(self, new_policy: P2) -> AgentGen<DP, P2, Comm>
    {
        AgentGen::<DP, P2, Comm>{
            state: self.state,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            actual_universal_score: self.actual_universal_score,
            comm: self.comm,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component
        }
    }

    pub fn do_replace_policy<P2: Policy<DP, StateType=P::StateType>>(self, new_policy: P2) -> (AgentGen<DP, P2, Comm>, P)
    {
        let p = self.policy;
        (AgentGen::<DP, P2, Comm>{
            state: self.state,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            actual_universal_score: self.actual_universal_score,
            comm: self.comm,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component
        }, p)
    }

    pub fn replace_comm(&mut self, mut comm: Comm) -> Comm{
        std::mem::swap(&mut self.comm, &mut comm);
        comm
    }
    pub fn swap_comms<P2: Policy<DP>>(&mut self, other: &mut AgentGen<DP, P2, Comm>)
    where <P2 as Policy<DP>>::StateType: ScoringInformationSet<DP>{
        std::mem::swap(&mut self.comm, &mut other.comm)
    }
    pub fn swap_comms_with_tracing<P2: Policy<DP>>(&mut self, other: &mut AgentGenT<DP, P2, Comm>)
    where <P2 as Policy<DP>>::StateType: ScoringInformationSet<DP>{
        std::mem::swap(&mut self.comm, &mut other.comm_mut())
    }

    pub(crate) fn comm_mut(&mut self) -> &mut Comm{
        &mut self.comm
    }

    pub fn change_id(&mut self, id: DP::AgentId){
        self.id = id
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
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>
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
StatefulAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
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
ActingAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

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
PolicyAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
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
Agent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    fn id(&self) -> DP::AgentId {
        self.id
    }

    fn change_id(&mut self, new_id: DP::AgentId) {
        self.id = new_id;
    }
}

impl<DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>> EnvRewardedAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
    fn current_universal_reward(&self) -> DP::UniversalReward {
        self.constructed_universal_reward.clone()
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &DP::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> DP::UniversalReward {
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
ResetAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::State) {
        self.state = initial_state;
        self.constructed_universal_reward = DP::UniversalReward::neutral();
        self.actual_universal_score = DP::UniversalReward::neutral();
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
InternalRewardedAgent<DP> for AgentGen<DP, P, Comm>
where <Self as StatefulAgent<DP>>::State: ScoringInformationSet<DP>,
      <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
    fn current_subjective_score(&self) ->  <<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType{
        self.state.current_subjective_score() + &self.explicit_subjective_reward_component
    }

    fn add_explicit_subjective_score(&mut self, explicit_reward: &<<Self as StatefulAgent<DP>>::State as ScoringInformationSet<DP>>::RewardType) {
        self.explicit_subjective_reward_component += explicit_reward
    }
}