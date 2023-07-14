use std::cell::Cell;
use sztorm::agent::Policy;
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::State;
use crate::common::RewardTable;
use crate::domain::{PrisonerAction, PrisonerCommit, PrisonerDomain, PrisonerError, PrisonerReward};
use crate::domain::PrisonerAction::{Betray, Cover};

#[derive(Clone, Debug)]
pub struct PrisonerState{
    previous_actions: Vec<PrisonerCommit>,
    reward_table: RewardTable,
    last_action: Cell<Option<PrisonerAction>>

}

impl PrisonerState{
    pub fn new(reward_table: RewardTable) -> Self{
        Self{reward_table, last_action: Cell::new(None), previous_actions: Vec::new()}
    }

    pub fn _select_action(&self, action: PrisonerAction){
        self.last_action.set(Some(action));
    }

    pub fn previous_actions(&self) -> &Vec<PrisonerCommit>{
        &self.previous_actions
    }
}

impl State<PrisonerDomain> for PrisonerState {
    fn update(&mut self, update: PrisonerCommit) -> Result<(), PrisonerError> {
        let last = self.last_action.get();
        if let Some(my_action) = last{
            if my_action == update.0{
                self.previous_actions.push(update);
                self.last_action.set(None);
                Ok(())
            } else{
                Err(PrisonerError::DifferentActionPerformed {chosen: my_action, logged: update.0})
            }
        } else {
            Err(PrisonerError::NoLastAction(update.0))
        }
    }
}

pub struct CoverPolicy{}

impl Policy<PrisonerDomain> for CoverPolicy{
    type StateType = PrisonerState;

    fn select_action(&self, state: &Self::StateType) -> Option<PrisonerAction> {
        state._select_action(Cover);
        Some(Cover)
    }
}

pub struct Forgive1Policy{}

impl Policy<PrisonerDomain> for Forgive1Policy{
    type StateType = PrisonerState;

    fn select_action(&self, state: &Self::StateType) -> Option<PrisonerAction> {
        let enemy_betrayals = state.previous_actions().iter().filter(| &step|{
            step.1 == Betray
        }).count();
        if enemy_betrayals > 1 {
            state._select_action(Betray);
            Some(Betray)
        } else {
            state._select_action(Cover);
            Some(Cover)
        }

    }
}

impl InformationSet<PrisonerDomain> for PrisonerState{
    type ActionIteratorType = [PrisonerAction;2];

    fn available_actions(&self) -> Self::ActionIteratorType {
        [Betray, Cover]
    }


    fn is_action_valid(&self, _action: &PrisonerAction) -> bool {
        true
    }
}

impl ScoringInformationSet<PrisonerDomain> for PrisonerState{
    type RewardType = PrisonerReward;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.previous_actions.iter().fold(0, |acc, x|{
            acc + self.reward_table.reward(x.0, x.1)
        })
    }
}

