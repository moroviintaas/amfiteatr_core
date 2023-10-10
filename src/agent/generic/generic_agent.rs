use std::marker::PhantomData;
use crate::agent::{
    CommunicatingAgent,
    ActingAgent,
    StatefulAgent,
    PolicyAgent,
    EnvRewardedAgent,
    Agent, ResetAgent,
    InternalRewardedAgent,
    AgentGenT
};
use crate::agent::info_set::{InformationSet, ScoringInformationSet};
use crate::agent::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::domain::{AgentMessage, EnvMessage, DomainParameters, Reward};

/// Generic agent implementing traits proposed in this crate.
/// This agent implements minimal functionality to work automatically with environment.
/// This agents does not collect trace of game, for are agent collecting it look for [AgentGenT](crate::agent::AgentGenT).
/// This agent can be built if used Policy operates on information set that is [`ScoringInformationSet`](crate::agent::ScoringInformationSet)
pub struct AgentGen<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
    /// Information Set (State as viewed by agent)
    information_set: <P as Policy<DP>>::InfoSetType,
    /// Communication endpoint, should be paired with environment's.
    comm: Comm,
    /// Agent's policy
    policy: P,
    _phantom: PhantomData<DP>,

    id: DP::AgentId,
    constructed_universal_reward: <DP as DomainParameters>::UniversalReward,
    actual_universal_score: <DP as DomainParameters>::UniversalReward,
    explicit_subjective_reward_component: <P::InfoSetType as ScoringInformationSet<DP>>::RewardType,
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>
    >
>
    AgentGen<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>
{


    pub fn new(id: DP::AgentId, state: <P as Policy<DP>>::InfoSetType, comm: Comm, policy: P) -> Self{
        Self{
            information_set: state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            actual_universal_score: Reward::neutral(),
            explicit_subjective_reward_component: <P::InfoSetType as ScoringInformationSet<DP>>::RewardType::neutral()
        }
    }

    /// Replaces information set in agent. Does not anything beside it.
    pub fn replace_info_set(&mut self, state: <P as Policy<DP>>::InfoSetType){
        self.information_set = state
    }

    /// Given new policy consumes this agent producing replacement agent (with moved internal state).
    /// New agent has now provided policy. Previous policy is dropped.
    /// # Example:
    /// ```
    /// use sztorm::agent::{AgentGen, RandomPolicy};
    /// use sztorm::comm::SyncCommEnv;
    /// use sztorm::demo::{DemoAgentID, DemoInfoSet, DemoPolicySelectFirst};
    /// let (_, comm) = SyncCommEnv::new_pair();
    /// let agent = AgentGen::new(DemoAgentID::Red, DemoInfoSet::new(10), comm, RandomPolicy::new());
    /// let agent_2 = agent.transform_replace_policy(DemoPolicySelectFirst{});
    /// ```
    pub fn transform_replace_policy<P2: Policy<DP, InfoSetType=P::InfoSetType>>(self, new_policy: P2) -> AgentGen<DP, P2, Comm>
    {
        AgentGen::<DP, P2, Comm>{
            information_set: self.information_set,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            actual_universal_score: self.actual_universal_score,
            comm: self.comm,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component
        }
    }
    /// Given new policy consumes this agent producing replacement agent (with moved internal state).
    /// New agent has now provided policy. Previous policy is returned as second element in tuple.
    /// # Example:
    /// ```
    /// use sztorm::agent::{AgentGen, RandomPolicy};
    /// use sztorm::comm::SyncCommEnv;
    /// use sztorm::demo::{DemoAgentID, DemoInfoSet, DemoPolicySelectFirst};
    /// let (_, comm) = SyncCommEnv::new_pair();
    /// let agent = AgentGen::new(DemoAgentID::Red, DemoInfoSet::new(10), comm, RandomPolicy::new());
    /// let (agent_2, old_policy) = agent.transform_replace_policy_ret(DemoPolicySelectFirst{});
    /// ```
    pub fn transform_replace_policy_ret<P2: Policy<DP, InfoSetType=P::InfoSetType>>(self, new_policy: P2) -> (AgentGen<DP, P2, Comm>, P)
    {
        let p = self.policy;
        (AgentGen::<DP, P2, Comm>{
            information_set: self.information_set,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            actual_universal_score: self.actual_universal_score,
            comm: self.comm,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component
        }, p)
    }

    /// Replaces communication endpoint returning old in return;
    pub fn replace_comm(&mut self, mut comm: Comm) -> Comm{
        std::mem::swap(&mut self.comm, &mut comm);
        comm
    }
    /// Using [`std::mem::swap`](::std::mem::swap) swaps communication endpoints between two instances.
    pub fn swap_comms<P2: Policy<DP>>(&mut self, other: &mut AgentGen<DP, P2, Comm>)
    where <P2 as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
        std::mem::swap(&mut self.comm, &mut other.comm)
    }
    /// Using [`std::mem::swap`](::std::mem::swap) swaps communication endpoints with instance of [`AgentGentT`](crate::agent::AgentGenT).
    pub fn swap_comms_with_tracing<P2: Policy<DP>>(&mut self, other: &mut AgentGenT<DP, P2, Comm>)
    where <P2 as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
        std::mem::swap(&mut self.comm, &mut other.comm_mut())
    }

    pub(crate) fn comm_mut(&mut self) -> &mut Comm{
        &mut self.comm
    }

    /// Changes agent's id.
    /// __Warning:__ id of agent should correspond with paired communication endpoint on the environment's side.
    pub fn change_id(&mut self, id: DP::AgentId){
        self.id = id
    }

    /*
    /// Adds current partial reward to actual score, and then neutralises universal reward
    fn commit_reward_to_score(&mut self){
        self.actual_universal_score += &self.constructed_universal_reward;
        self.constructed_universal_reward = DP::UniversalReward::neutral();
    }

     */
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>
    >
>
    CommunicatingAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>
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
        Error=CommError<DP>
    >
>
StatefulAgent<DP> for AgentGen<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
    type InfoSetType = <P as Policy<DP>>::InfoSetType;

    fn update(&mut self, state_update: DP::UpdateType) -> Result<(), DP::GameErrorType> {
        self.information_set.update(state_update)
    }

    fn info_set(&self) -> &Self::InfoSetType {
        &self.information_set
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
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    fn take_action(&mut self) -> Option<DP::ActionType> {
        //self.commit_reward_to_score();
        self.commit_partial_rewards();
        self.policy.select_action(&self.information_set)

    }

    fn finalize(&mut self) {
        self.commit_partial_rewards();
        //self.commit_reward_to_score();
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
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
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
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

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
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
    fn current_universal_reward(&self) -> DP::UniversalReward {
        self.constructed_universal_reward.clone()
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &DP::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> DP::UniversalReward {
        self.actual_universal_score.clone() + &self.constructed_universal_reward
    }

    fn commit_partial_rewards(&mut self) {
        self.actual_universal_score += &self.constructed_universal_reward;
        self.constructed_universal_reward = DP::UniversalReward::neutral();
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
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::InfoSetType) {
        self.information_set = initial_state;
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
where <Self as StatefulAgent<DP>>::InfoSetType: ScoringInformationSet<DP>,
      <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
    fn current_subjective_score(&self) ->  <<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>::RewardType{
        self.information_set.current_subjective_score() + &self.explicit_subjective_reward_component
    }

    fn add_explicit_subjective_score(&mut self, explicit_reward: &<<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>::RewardType) {
        self.explicit_subjective_reward_component += explicit_reward
    }
}