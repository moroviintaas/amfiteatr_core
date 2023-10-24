use std::marker::PhantomData;

use crate::agent::{ActingAgent, Agent, CommunicatingAgent, AgentTrajectory, AgentTraceStep, Policy, PolicyAgent, ResetAgent, EnvRewardedAgent, StatefulAgent, TracingAgent, InternalRewardedAgent, AgentGen, InformationSet};
use crate::agent::info_set::ScoringInformationSet;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::domain::{AgentMessage, DomainParameters, EnvMessage, Reward};


/// Generic agent implementing traits proposed in this crate.
/// This agent implements minimal functionality to work automatically with environment.
/// This agents  collects trace of game, for are agent not collecting it look for [AgentGen](crate::agent::AgentGen).
/// This agent can be built if used Policy operates on information set that is [`ScoringInformationSet`](crate::agent::ScoringInformationSet)
pub struct AgentGenT<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{


    state: <P as Policy<DP>>::InfoSetType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<DP>,

    id: DP::AgentId,
    constructed_universal_reward: <DP as DomainParameters>::UniversalReward,
    committed_universal_score: <DP as DomainParameters>::UniversalReward,

    game_trajectory: AgentTrajectory<DP, P::InfoSetType>,
    last_action: Option<DP::ActionType>,
    state_before_last_action: Option<<P as Policy<DP>>::InfoSetType>,
    explicit_subjective_reward_component: <P::InfoSetType as ScoringInformationSet<DP>>::RewardType,
}

impl <DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    pub fn new(id: DP::AgentId, state: <P as Policy<DP>>::InfoSetType, comm: Comm, policy: P) -> Self{
        Self{state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            committed_universal_score: Reward::neutral(),
            game_trajectory: AgentTrajectory::new(),
            state_before_last_action: None,
            last_action: None,
            explicit_subjective_reward_component: <P::InfoSetType as ScoringInformationSet<DP>>::RewardType::neutral()
        }
    }

    /// Given new policy consumes this agent producing replacement agent (with moved internal state).
    /// New agent has now provided policy. Previous policy is dropped.
    /// # Example:
    /// ```
    /// use amfi::agent::{AgentGenT, RandomPolicy};
    /// use amfi::comm::SyncCommEnv;
    /// use amfi::demo::{DemoAgentID, DemoInfoSet, DemoPolicySelectFirst};
    /// let (_, comm) = SyncCommEnv::new_pair();
    /// let agent = AgentGenT::new(DemoAgentID::Red, DemoInfoSet::new(10), comm, RandomPolicy::new());
    /// let agent_2 = agent.transform_replace_policy(DemoPolicySelectFirst{});
    /// ```
    pub fn transform_replace_policy<P2: Policy<DP, InfoSetType=P::InfoSetType>>(self, new_policy: P2) -> AgentGenT<DP, P2, Comm>
    {
        AgentGenT::<DP, P2, Comm>{
            state: self.state,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            committed_universal_score: self.committed_universal_score,
            comm: self.comm,
            last_action: self.last_action,
            state_before_last_action: self.state_before_last_action,
            game_trajectory: self.game_trajectory,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component
        }
    }


    /// Given new policy consumes this agent producing replacement agent (with moved internal state).
    /// New agent has now provided policy. Previous policy is returned as second element in tuple.
    /// # Example:
    /// ```
    /// use amfi::agent::{AgentGenT, RandomPolicy};
    /// use amfi::comm::SyncCommEnv;
    /// use amfi::demo::{DemoAgentID, DemoInfoSet, DemoPolicySelectFirst};
    /// let (_, comm) = SyncCommEnv::new_pair();
    /// let agent = AgentGenT::new(DemoAgentID::Red, DemoInfoSet::new(10), comm, RandomPolicy::new());
    /// let (agent_2, old_policy) = agent.transform_replace_policy_ret(DemoPolicySelectFirst{});
    /// ```
    pub fn transform_replace_policy_ret<P2: Policy<DP, InfoSetType=P::InfoSetType>>(self, new_policy: P2) -> (AgentGenT<DP, P2, Comm>, P)
    {
        let p = self.policy;
        (AgentGenT::<DP, P2, Comm>{
            state: self.state,
            policy: new_policy,
            _phantom: Default::default(),
            id: self.id,
            constructed_universal_reward: self.constructed_universal_reward,
            comm: self.comm,
            explicit_subjective_reward_component: self.explicit_subjective_reward_component,
            last_action: self.last_action,
            state_before_last_action: self.state_before_last_action,
            game_trajectory: self.game_trajectory,
            committed_universal_score: self.committed_universal_score,
        }, p)
    }

    /// Replaces communication endpoint returning old in return;
    pub fn replace_comm(&mut self, mut comm: Comm) -> Comm{
        std::mem::swap(&mut self.comm, &mut comm);
        comm
    }
    /// Using [`std::mem::swap`](::std::mem::swap) swaps communication endpoints between two instances.
    pub fn swap_comms<P2: Policy<DP>>(&mut self, other: &mut AgentGenT<DP, P2, Comm>)
    where <P2 as Policy<DP>>::InfoSetType: ScoringInformationSet<DP> + Clone{
        std::mem::swap(&mut self.comm, &mut other.comm)
    }

    /// Using [`std::mem::swap`](::std::mem::swap) swaps communication endpoints with instance of [`AgentGent`](crate::agent::AgentGen).
    pub fn swap_comms_with_basic<P2: Policy<DP>>(&mut self, other: &mut AgentGen<DP, P2, Comm>)
    where <P2 as Policy<DP>>::InfoSetType: ScoringInformationSet<DP> + Clone{
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

}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
Agent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    fn id(&self) -> DP::AgentId {
        self.id
    }

    fn change_id(&mut self, new_id: DP::AgentId) {
        self.id = new_id;
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
    CommunicatingAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP> + Clone{

    type CommunicationError = CommError<DP>;

    fn send(&mut self, message: AgentMessage<DP>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<DP>, Self::CommunicationError> {
        self.comm.receive_blocking()
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
StatefulAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    type InfoSetType = <P as Policy<DP>>::InfoSetType;

    fn update(&mut self, state_update: DP::UpdateType) -> Result<(), DP::GameErrorType> {
        self.state.update(state_update)
    }

    fn info_set(&self) -> &Self::InfoSetType {
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
ActingAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP> + Clone{

    /// Firstly, agent commits last step to stack.
    fn take_action(&mut self) -> Option<DP::ActionType> {
        self.commit_trace();

        let action = self.policy.select_action(&self.state);
        self.last_action = action.clone();
        self.state_before_last_action = Some(self.state.clone());
        action
    }

    fn finalize(&mut self) {
        self.commit_trace();
        self.state_before_last_action = Some(self.state.clone())
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
TracingAgent<DP, <P as Policy<DP>>::InfoSetType> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP> ,
//for <'a> &'a<DP as DomainParameters>::UniversalReward: Sub<&'a <DP as DomainParameters>::UniversalReward, Output=<DP as DomainParameters>::UniversalReward>,
//for<'a> &'a <<P as Policy<DP>>::StateType as ScoringInformationSet<DP>>::RewardType: Sub<&'a  <<P as Policy<DP>>::StateType as ScoringInformationSet<DP>>::RewardType, Output = <<P as Policy<DP>>::StateType as ScoringInformationSet<DP>>::RewardType>
{
    fn reset_trajectory(&mut self) {
        self.game_trajectory.clear();
        self.last_action = None;
    }

    fn take_trajectory(&mut self) -> AgentTrajectory<DP, <P as Policy<DP>>::InfoSetType> {
        std::mem::take(&mut self.game_trajectory)
    }

    fn game_trajectory(&self) -> &AgentTrajectory<DP, <P as Policy<DP>>::InfoSetType> {
        &self.game_trajectory
    }

    /// Commits information set change to trajectory. Adds [
    fn commit_trace(&mut self) {
        if let Some(prev_action) = self.last_action.take(){

           /* let universal_score_before_update = self.committed_universal_score.clone();

            self.committed_universal_score += &self.constructed_universal_reward;
            let universal_score_after_update = self.committed_universal_score.clone();
            self.constructed_universal_reward = DP::UniversalReward::neutral();
            */
            let universal_score_before_update = self.committed_universal_score.clone();
            self.commit_partial_rewards();
            let universal_score_after_update = self.committed_universal_score.clone();
            let initial_state = self.state_before_last_action.take().unwrap();
            let subjective_score_before_update = initial_state.current_subjective_score();
            let subjective_score_after_update = self.state.current_subjective_score() + &self.explicit_subjective_reward_component;


            self.game_trajectory.push_trace_step(
                AgentTraceStep::new(
                    initial_state,
                    prev_action,
                    universal_score_before_update,
                    universal_score_after_update,
                    subjective_score_before_update,
                    subjective_score_after_update,
                    ));

        }
    }

    fn explicit_add_subjective_reward(&mut self, explicit: <<P as Policy<DP>>::InfoSetType as ScoringInformationSet<DP>>::RewardType) {
        self.explicit_subjective_reward_component += &explicit
    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
PolicyAgent<DP> for AgentGenT<DP, P, Comm>
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
EnvRewardedAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    fn current_universal_reward(&self) -> DP::UniversalReward {
        self.constructed_universal_reward.clone()
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &DP::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> DP::UniversalReward {
        self.committed_universal_score.clone() + &self.constructed_universal_reward
    }
    fn commit_partial_rewards(&mut self) {
        self.committed_universal_score += &self.constructed_universal_reward;
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
ResetAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::InfoSetType) {
        self.state = initial_state;
        self.game_trajectory.clear();
        self.constructed_universal_reward = DP::UniversalReward::neutral();
        self.committed_universal_score = DP::UniversalReward::neutral();
        self.state_before_last_action = None;
        self.last_action = None;

    }
}

impl<
    DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
InternalRewardedAgent<DP> for AgentGenT<DP, P, Comm>
where <Self as StatefulAgent<DP>>::InfoSetType: ScoringInformationSet<DP>,
<P as Policy<DP>>::InfoSetType: ScoringInformationSet<DP>{
    fn current_subjective_score(&self) ->  <<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>::RewardType{
        self.state.current_subjective_score() + &self.explicit_subjective_reward_component
    }

    fn add_explicit_subjective_score(&mut self, explicit_reward: &<<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>::RewardType) {
        self.explicit_subjective_reward_component += explicit_reward
    }
}