use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::protocol::DomainParameters;
use crate::state::agent::{ScoringInformationSet};


#[derive(Clone, Debug)]
pub struct AgentTrace<DP: DomainParameters, S: ScoringInformationSet<DP>> {
    initial_state: S,
    taken_action: DP::ActionType,
    initial_universal_state_score: DP::UniversalReward,
    updated_universal_state_score: DP::UniversalReward,

    initial_subjective_state_score: S::RewardType,
    updated_subjective_state_score: S::RewardType


}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> AgentTrace<DP, S>
//where for <'a> &'a<DP as DomainParameters>::UniversalReward: Sub<&'a <DP as DomainParameters>::UniversalReward, Output=<DP as DomainParameters>::UniversalReward>,
//    for<'a> &'a <S as ScoringInformationSet<DP>>::RewardType: Sub<&'a  <S as ScoringInformationSet<DP>>::RewardType, Output = <S as ScoringInformationSet<DP>>::RewardType>

{
    pub fn new(
        initial_state: S,
        taken_action: DP::ActionType,
        initial_universal_state_score: DP::UniversalReward,
        updated_universal_state_score: DP::UniversalReward,
        initial_subjective_state_score: S::RewardType,
        updated_subjective_state_score: S::RewardType
    )-> Self{
        Self {
            initial_state,
            taken_action,
            initial_universal_state_score,
            updated_universal_state_score,
            initial_subjective_state_score,
            updated_subjective_state_score
        }
    }

    pub fn step_state(&self) -> &S{
        &self.initial_state
    }
    pub fn taken_action(&self) -> &DP::ActionType{
        &self.taken_action
    }

    pub fn step_subjective_reward(&self) -> S::RewardType{
        let n = self.updated_subjective_state_score.clone();
        n - &self.initial_subjective_state_score

    }
    pub fn step_universal_reward(&self) -> DP::UniversalReward{
        let n = self.updated_universal_state_score.clone();
        n - &self.initial_universal_state_score
    }

    pub fn universal_score_before(&self) -> &DP::UniversalReward{
        &self.initial_universal_state_score
    }
    pub fn subjective_score_before(&self) -> &S::RewardType{
        &self.initial_subjective_state_score
    }


    pub fn universal_score_after(&self) -> &DP::UniversalReward{
        &self.updated_universal_state_score
    }

    pub fn subjective_score_after(&self) -> &S::RewardType{
        &self.updated_subjective_state_score
    }



    pub fn s_a_r_universal(&self) -> (&S, &DP::ActionType, DP::UniversalReward) {
        (self.step_state(), self.taken_action(), self.step_universal_reward())
    }
    pub fn s_a_r_subjective(&self) -> (&S, &DP::ActionType, S::RewardType) {
        (self.step_state(), self.taken_action(), self.step_subjective_reward())
    }
    //pub fn s_a_r(&self, source:S RewardSource) -
}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> Display for AgentTrace<DP, S>
where
    S: Display,
    <DP as DomainParameters>::UniversalReward: Display,
    <DP as DomainParameters>::ActionType: Display,
    <S as  ScoringInformationSet<DP>>::RewardType : Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[State: {} ][From Score: U = {} | A = {}][Action: {} ][To Score: U = {} | A = {}]",
            self.initial_state,

            self.initial_universal_state_score,
            self.initial_subjective_state_score,
            self.taken_action,
            self.updated_universal_state_score,
            self.updated_subjective_state_score
        )
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
    pub fn push_trace(&mut self, trace_line: AgentTrace<DP, S>){
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

    pub fn is_empty(&self) -> bool{
        self.list().is_empty()
    }
}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> Index<usize> for AgentTrajectory<DP, S>{
    type Output = AgentTrace<DP, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.trace[index]
    }
}