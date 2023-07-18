use std::collections::HashMap;
use enum_map::EnumMap;
use sztorm::agent::AgentIdentifier;
use sztorm::env::{EnvironmentState, EnvironmentStateUniScore};
use sztorm::protocol::DomainParameters;
use crate::common::RewardTable;
use crate::domain::{PRISONERS, PrisonerAction,  PrisonerDomain, PrisonerError, PrisonerId, PrisonerUpdate};
use crate::domain::PrisonerError::{ActionAfterGameOver, ActionOutOfOrder};



#[derive(Clone, Debug)]
pub struct PrisonerEnvState{
    previous_actions: Vec<EnumMap<PrisonerId, Option<PrisonerAction>>>,
    //last_actions: HashMap<PrisonerId, Option<PrisonerAction>>,
    last_round_actions: EnumMap<PrisonerId, Option<PrisonerAction>>,
    reward_table: RewardTable,
    target_rounds: usize,

}

impl PrisonerEnvState{
    pub fn new(reward_table: RewardTable, number_of_rounds: usize) -> Self{
        Self{
            previous_actions: Vec::with_capacity(number_of_rounds),
            last_round_actions: EnumMap::default(),
            reward_table,
            target_rounds: number_of_rounds
        }
    }
}




impl EnvironmentState<PrisonerDomain> for PrisonerEnvState{
    type Updates = Vec<(PrisonerId, PrisonerUpdate)>;

    fn current_player(&self) -> Option<PrisonerId> {
        if self.previous_actions.len() >= self.target_rounds{
            None
        } else{
            for i in PRISONERS{
                if self.last_round_actions[i].is_none(){
                    return Some(i)
                }
            }
            None
        }

    }





    fn is_finished(&self) -> bool {
        self.previous_actions.len() >= self.target_rounds
    }

    fn forward(&mut self, agent: PrisonerId, action: PrisonerAction) -> Result<Self::Updates, PrisonerError> {
        if self.is_finished(){
            return Err(ActionAfterGameOver(agent));
        }
        match self.last_round_actions[agent]{
            None => {
                self.last_round_actions[agent] = Some(action);


            },
            Some(_) => {
                return Err(ActionOutOfOrder(agent));
            }
        };
        for agent in PRISONERS {
            if self.last_round_actions[agent].is_none(){
                return Ok(Vec::default());
            }
        }

        let a0 = self.last_round_actions[0].unwrap();
        let a1 = self.last_round_actions[1].unwrap();
        self.previous_actions.push(self.last_round_actions);
        self.last_round_actions[0] = None;
        self.last_round_actions[1] = None;

        let mut updates = Vec::new();
        updates.push((0, PrisonerUpdate{
            own_action: a0,
            other_prisoner_action: a1
        }));
        updates.push((1, PrisonerUpdate{
            own_action: a1,
            other_prisoner_action: a0
        }));

        Ok(updates)

    }
}

impl EnvironmentStateUniScore<PrisonerDomain> for PrisonerEnvState{
    fn state_score_of_player(&self, agent: &PrisonerId) -> <PrisonerDomain as DomainParameters>::UniversalReward {
        let other = (agent+1) & 0x1;
        self.previous_actions.iter().fold(0, |acc,x|{
            self.reward_table.reward(x[*agent].unwrap(), x[other].unwrap())
        })
    }
}