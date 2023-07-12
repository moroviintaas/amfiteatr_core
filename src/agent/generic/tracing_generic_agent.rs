use std::marker::PhantomData;
use crate::agent::{ActingAgent, Agent, CommunicatingAgent, GameTrace, GameTraceLine, Policy, PolicyAgent, ResetAgent, RewardedAgent, StatefulAgent, TracingAgent};
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::protocol::{AgentMessage, DomainParameters, EnvMessage};
use crate::Reward;
use crate::state::agent::ScoringInformationSet;
use crate::state::State;

pub struct AgentGenT<
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

    game_trajectory: GameTrace<DP, P::StateType>,
    last_action: Option<DP::ActionType>,
    state_before_last_action: Option<<P as Policy<DP>>::StateType>,
}

impl <DP: DomainParameters,
    P: Policy<DP>,
    Comm: CommEndpoint<
        OutwardType=AgentMessage<DP>,
        InwardType=EnvMessage<DP>,
        Error=CommError<DP>>>
AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    pub fn new(id: DP::AgentId, state: <P as Policy<DP>>::StateType, comm: Comm, policy: P) -> Self{
        Self{state,
            comm,
            policy,
            _phantom:PhantomData::default(),
            id,
            constructed_universal_reward: Reward::neutral(),
            actual_universal_score: Reward::neutral(),
            game_trajectory: GameTrace::new(),
            state_before_last_action: None,
            last_action: None,
        }
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
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    fn id(&self) -> DP::AgentId {
        self.id
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
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

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
StatefulAgent<DP> for AgentGenT<DP, P, Comm>
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
ActingAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

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
TracingAgent<DP, <P as Policy<DP>>::StateType> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{
    fn reset_trace(&mut self) {
        self.game_trajectory.clear();
        self.last_action = None;
    }

    fn game_trajectory(&self) -> &GameTrace<DP, <P as Policy<DP>>::StateType> {
        &self.game_trajectory
    }

    fn commit_trace(&mut self) {
        if let Some(prev_action) = self.last_action.take(){
            //self.trace.push((self.last_action_state.take().unwrap(), prev_action, self.state.current_score()- std::mem::take(&mut self.last_action_accumulated_reward)))
            let prev_subjective_score = match &self.state_before_last_action{
                None => Reward::neutral(),
                Some(state) => state.current_subjective_score()
            };
            let push_universal_reward = std::mem::replace(&mut self.constructed_universal_reward, Reward::neutral());
            self.actual_universal_score  += &push_universal_reward;
            self.game_trajectory.push_line(
                GameTraceLine::new(
                    self.state_before_last_action.take().unwrap(),
                    prev_action,
                    self.state.current_subjective_score() - prev_subjective_score,
                    push_universal_reward));

        }
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
RewardedAgent<DP> for AgentGenT<DP, P, Comm>
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
ResetAgent<DP> for AgentGenT<DP, P, Comm>
where <P as Policy<DP>>::StateType: ScoringInformationSet<DP>{

    fn reset(&mut self, initial_state: <Self as StatefulAgent<DP>>::State) {
        self.state = initial_state;
        self.constructed_universal_reward = DP::UniversalReward::neutral();
        self.actual_universal_score = DP::UniversalReward::neutral();
    }
}