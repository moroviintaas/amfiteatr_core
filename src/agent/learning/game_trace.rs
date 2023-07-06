use crate::protocol::DomainParameters;
use crate::{InformationSet};

pub struct GameTraceLine<DP: DomainParameters, S: InformationSet<DP>> {
    initial_state: S,
    taken_action: DP::ActionType,
    immediate_subjective_reward: S::RewardType,
    immediate_universal_reward: DP::UniversalReward

}

impl<DP: DomainParameters, S: InformationSet<DP>> GameTraceLine<DP, S>{
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

pub struct GameTrace<DP: DomainParameters, S: InformationSet<DP>> {


    //top_state: S,
    trace: Vec<GameTraceLine<DP, S>>

}

impl<DP: DomainParameters, S: InformationSet<DP>> Default for GameTrace<DP, S>{
    fn default() -> Self {
        Self{trace: Default::default()}
    }
}
impl<DP: DomainParameters, S: InformationSet<DP>> GameTrace<DP, S>
{

    pub fn new() -> Self{
        Self{trace: Default::default()}
    }
    /*pub fn register_line(&mut self, state: S, action: DP::ActionType, reward_for_action: S::RewardType){
        self.trace.push(GameTraceLine::new(state, action, reward_for_action));

    }*/
    pub fn push_line(&mut self, trace_line: GameTraceLine<DP, S>){
        self.trace.push(trace_line);
    }
    pub fn clear(&mut self){
        self.trace.clear();
    }

    pub fn trace(&self) -> &Vec<GameTraceLine<DP, S>>{
        &self.trace
    }

    pub fn pop_step(&mut self) -> Option<GameTraceLine<DP, S>>{
        self.trace.pop()
    }

}