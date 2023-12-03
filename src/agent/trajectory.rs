use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::agent::info_set::ScoringInformationSet;
use crate::domain::DomainParameters;

/// This struct contains information about _information set (game state from view of agent)_
/// before taken action along with taken action and saved score before and after taking action.
/// __Note__ scores after taking action are __not__ measured in the moment just after taking action,
/// but just before taking subsequent action i.e. this is _information set_ for __next__ step.
///
#[derive(Clone, Debug)]
pub struct AgentTraceStep<DP: DomainParameters, S: ScoringInformationSet<DP>> {
    initial_info_set: S,
    taken_action: DP::ActionType,
    initial_universal_state_score: DP::UniversalReward,
    updated_universal_state_score: DP::UniversalReward,

    initial_subjective_state_score: S::RewardType,
    updated_subjective_state_score: S::RewardType


}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> AgentTraceStep<DP, S>
//where for <'a> &'a<DP as DomainParameters>::UniversalReward: Sub<&'a <DP as DomainParameters>::UniversalReward, Output=<DP as DomainParameters>::UniversalReward>,
//    for<'a> &'a <S as ScoringInformationSet<DP>>::RewardType: Sub<&'a  <S as ScoringInformationSet<DP>>::RewardType, Output = <S as ScoringInformationSet<DP>>::RewardType>

{
    /// Constructor of AgentTraceStep
    /// # Args:
    /// - `initial_info_set`: Information set before taken action
    /// - `taken_action`: Performed action (in the state of `initial_info_set`)
    /// - `initial_universal_state_score`: score before taking action, i.e. at the moment of `initial_info_set`,
    ///  taken from environment
    /// - `updated_universal_state_score`: score after taking action
    /// taken from environment
    /// - `initial_subjective_state_score`: score before taking action, i.e. at the moment of `initial_info_set`
    /// measured on information set
    /// - `updated_universal_state_score`: score after taking action - measured on information set
    pub fn new(

        initial_info_set: S,
        taken_action: DP::ActionType,
        initial_universal_state_score: DP::UniversalReward,
        updated_universal_state_score: DP::UniversalReward,
        initial_subjective_state_score: S::RewardType,
        updated_subjective_state_score: S::RewardType
    ) -> Self{
        Self {
            initial_info_set,
            taken_action,
            initial_universal_state_score,
            updated_universal_state_score,
            initial_subjective_state_score,
            updated_subjective_state_score
        }
    }

    /// Returns reference to information set trapped for this step (before action taken)
    pub fn step_info_set(&self) -> &S{
        &self.initial_info_set
    }

    /// Return reference to taken action in this step
    pub fn taken_action(&self) -> &DP::ActionType{
        &self.taken_action
    }

    /// Returns subjective reward for taken action - difference between score before __next__ action,
    /// and score before taking __this__ action. This relates to reward received from environment.
    pub fn step_subjective_reward(&self) -> S::RewardType{
        let n = self.updated_subjective_state_score.clone();
        n - &self.initial_subjective_state_score

    }
    /// Returns subjective reward for taken action - difference between score before __next__ action,
    /// and score before taking __this__ action. This relates to reward measured on information set.
    pub fn step_universal_reward(&self) -> DP::UniversalReward{
        let n = self.updated_universal_state_score.clone();
        n - &self.initial_universal_state_score
    }

    /// Returns reference universal score (sourced from environment)
    pub fn universal_score_before(&self) -> &DP::UniversalReward{
        &self.initial_universal_state_score
    }
    /// Returns reference to score sourced from information set (before action)
    pub fn subjective_score_before(&self) -> &S::RewardType{
        &self.initial_subjective_state_score
    }


    /// Returns reference to universal score (sourced from environment) after taking action (and optional actions of other players
    pub fn universal_score_after(&self) -> &DP::UniversalReward{
        &self.updated_universal_state_score
    }

    /// Returns reference to subjective score (sourced from information set) after taking action (and optional actions of other players
    pub fn subjective_score_after(&self) -> &S::RewardType{
        &self.updated_subjective_state_score
    }



    /// Returns tuple of respectively: reference to information set, reference to taken action, reward for taken action (sourced from environment)
    pub fn s_a_r_universal(&self) -> (&S, &DP::ActionType, DP::UniversalReward) {
        (self.step_info_set(), self.taken_action(), self.step_universal_reward())
    }

    /// Returns tuple of respectively: reference to information set, reference to taken action, reward for taken action (sourced from information set)
    pub fn s_a_r_subjective(&self) -> (&S, &DP::ActionType, S::RewardType) {
        (self.step_info_set(), self.taken_action(), self.step_subjective_reward())
    }
    //pub fn s_a_r(&self, source:S RewardSource) -
}

impl<DP: DomainParameters, S: ScoringInformationSet<DP>> Display for AgentTraceStep<DP, S>
where
    S: Display,
    <DP as DomainParameters>::UniversalReward: Display,
    <DP as DomainParameters>::ActionType: Display,
    <S as  ScoringInformationSet<DP>>::RewardType : Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[State: {} ][From Score: U = {} | A = {}][Action: {} ][To Score: U = {} | A = {}]",
               self.initial_info_set,

               self.initial_universal_state_score,
               self.initial_subjective_state_score,
               self.taken_action,
               self.updated_universal_state_score,
               self.updated_subjective_state_score
        )
    }
}


/// Trajectory of game from the view of agent. Currently it is stack of independent
/// trace steps (usually [`AgentTraceStep`](crate::agent::AgentTraceStep)) in struct of vector (wrapped around
/// for `Display` purpose).
/// > However in the future it may be structure better optimised in memory -
/// without redundancy of scores now most scores are stored doubled - once as score after action in step
/// and second time in the initial info set for next step.
pub struct AgentTrajectory<Tr> {


    //top_state: S,
    pub trace: Vec<Tr>

}
pub type StdAgentTrajectory<DP, IS> = AgentTrajectory<AgentTraceStep<DP, IS>>;
impl<Tr> Default for AgentTrajectory<Tr>{
    fn default() -> Self {
        Self{trace: Default::default()}
    }
}
impl<Tr> AgentTrajectory<Tr>
{


    pub fn new() -> Self{
        Self{trace: Default::default()}
    }
    /*pub fn register_line(&mut self, state: S, action: DP::ActionType, reward_for_action: S::RewardType){
        self.trace.push(GameTraceLine::new(state, action, reward_for_action));

    }*/

    /// Pushes trace step on the end of trajectory.
    pub fn push_trace_step(&mut self, trace_step: Tr){
        self.trace.push(trace_step);
    }
    /// Clears trajectory using [`Vec::clear()`](std::vec::Vec::clear)
    pub fn clear(&mut self){
        self.trace.clear();
    }

    /// Returns reference to `Vec` inside the structure.
    pub fn list(&self) -> &Vec<Tr>{
        &self.trace
    }

    /// Pops step from trajectory using [`Vec::pop()`](std::vec::Vec::pop)
    pub fn pop_step(&mut self) -> Option<Tr>{
        self.trace.pop()
    }


    pub fn is_empty(&self) -> bool{
        self.list().is_empty()
    }
}

impl<Tr> Index<usize> for AgentTrajectory<Tr>{
    type Output = Tr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.trace[index]
    }
}