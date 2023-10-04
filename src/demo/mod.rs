use std::fmt::{Debug, Display, Formatter};
use rand::{random, Rng, thread_rng};
use rand::distributions::Uniform;
use crate::agent::{AgentActionPair, AgentIdentifier};
use crate::demo::DemoAgentID::Blue;
use crate::domain::{Action, DomainParameters};
use crate::env::{EnvironmentState, ScoreEnvironment};
use crate::error::SztormError;
use rand::distributions::Distribution;
use crate::state::agent::InformationSet;

#[derive(Clone, Debug)]
pub struct DemoAction(u8);
impl Display for DemoAction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Action for DemoAction{}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, )]
pub enum DemoAgentID{
    Blue,
    Red
}
impl Display for DemoAgentID{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



impl AgentIdentifier for DemoAgentID{}

#[derive(Copy, Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub struct DemoError{}
impl Display for DemoError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DemoError")
    }
}



#[derive(Clone, Debug)]
pub struct DemoParams{}

impl DomainParameters for DemoParams{
    type ActionType = DemoAction;
    type GameErrorType = DemoError;
    type UpdateType = (DemoAgentID, DemoAction, f32);
    type AgentId = DemoAgentID;
    type UniversalReward = f32;
}

#[derive(Clone, Debug)]
pub struct DemoState{
    ceilings: Vec<f32>,
    max_rounds: u32,
    rewards: Vec<f32>
}

impl DemoState{
    pub fn new(ceilings: Vec<f32>, max_rounds: u32) -> Self{
        Self{ceilings, max_rounds, rewards: Vec::default()}
    }
}
impl EnvironmentState<DemoParams> for DemoState{
    type Updates = Vec<(DemoAgentID, (DemoAgentID, DemoAction, f32))>;

    fn current_player(&self) -> Option<DemoAgentID> {
        Some(Blue)
    }

    fn is_finished(&self) -> bool {
        self.rewards.len()  >= self.max_rounds as usize
    }

    fn forward(&mut self, agent: DemoAgentID, action: DemoAction) -> Result<Self::Updates, DemoError> {
        if action.0 as usize > self.ceilings.len(){
            return Err(DemoError{})
        }
        let mut r = thread_rng();
        let mut d = Uniform::new(0.0, self.ceilings[action.0 as usize]);
        let reward: f32 = d.sample(&mut r);
        self.rewards.push(reward);

        Ok(vec![(agent, (agent, action.clone(), reward))])

    }
}



#[derive(Clone, Debug)]
pub struct DemoInfoSet{
    pub number_of_bandits: usize,
    rewards: Vec<f32>
}

impl DemoInfoSet{
    pub fn new(number_of_bandits: usize) -> Self{
        Self{
            number_of_bandits,
            rewards: Vec::new()
        }
    }
}

impl InformationSet<DemoParams> for DemoInfoSet{
    type ActionIteratorType = Vec<DemoAction>;

    fn available_actions(&self) -> Self::ActionIteratorType {
        let mut v = Vec::with_capacity(self.number_of_bandits);
        for i in 0..self.number_of_bandits as u8{
            v.push(DemoAction(i));
        }
        v
    }

    fn is_action_valid(&self, action: &DemoAction) -> bool {
        (action.0 as usize) < self.number_of_bandits
    }

    fn update(&mut self, _update: (DemoAgentID, DemoAction, f32)) -> Result<(), DemoError> {
        Ok(())
    }
}