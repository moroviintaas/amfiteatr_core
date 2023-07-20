use std::fmt::{Display, Formatter};
use crate::protocol::DomainParameters;
use crate::state::agent::{ScoringInformationSet};


#[derive(Clone, Debug)]
pub struct AgentTrace<DP: DomainParameters, S: ScoringInformationSet<DP>> {
    initial_state: S,
    taken_action: DP::ActionType,
    immediate_subjective_reward: S::RewardType,
    immediate_universal_reward: DP::UniversalReward

}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> AgentTrace<DP, S>{
    pub fn new(initial_state: S, taken_action: DP::ActionType, immediate_subjective_reward: S::RewardType, immediate_universal_reward: DP::UniversalReward) -> Self{
        Self{initial_state, taken_action, immediate_subjective_reward, immediate_universal_reward }
    }

    pub fn step_state(&self) -> &S{
        &self.initial_state
    }
    pub fn taken_action(&self) -> &DP::ActionType{
        &self.taken_action
    }
    pub fn step_subjective_reward(&self) -> &S::RewardType{
        &self.immediate_subjective_reward
    }
    pub fn step_universal_reward(&self) -> &DP::UniversalReward{
        &self.immediate_universal_reward
    }
    pub fn borrowed_tuple(&self) -> (&S, &DP::ActionType, &S::RewardType) {
        (self.step_state(), self.taken_action(), self.step_subjective_reward())
    }
}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> Display for AgentTrace<DP, S>
where
    S: Display,
    <DP as DomainParameters>::UniversalReward: Display,
    <DP as DomainParameters>::ActionType: Display,
    <S as  ScoringInformationSet<DP>>::RewardType : Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[State: {} ][Action: {} ][Score change: U= {} |A= {}]",
            self.initial_state,
            self.taken_action,
            self.immediate_universal_reward,
            self.immediate_subjective_reward)
    }
}

pub struct AgentTrajectory<DP: DomainParameters, S: ScoringInformationSet<DP>> {


    //top_state: S,
    trace: Vec<AgentTrace<DP, S>>

}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> Default for AgentTrajectory<DP, S>{
    fn default() -> Self {
        Self{trace: Default::default()}
    }
}
impl<DP: DomainParameters, S: ScoringInformationSet<DP>> AgentTrajectory<DP, S>
{

    pub fn new() -> Self{
        Self{trace: Default::default()}
    }
    /*pub fn register_line(&mut self, state: S, action: DP::ActionType, reward_for_action: S::RewardType){
        self.trace.push(GameTraceLine::new(state, action, reward_for_action));

    }*/
    pub fn push_line(&mut self, trace_line: AgentTrace<DP, S>){
        self.trace.push(trace_line);
    }
    pub fn clear(&mut self){
        self.trace.clear();
    }

    pub fn list(&self) -> &Vec<AgentTrace<DP, S>>{
        &self.trace
    }

    pub fn pop_step(&mut self) -> Option<AgentTrace<DP, S>>{
        self.trace.pop()
    }

}