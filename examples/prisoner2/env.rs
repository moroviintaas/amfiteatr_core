use std::collections::HashMap;
use sztorm::protocol::DomainParameters;
use crate::common::RewardTable;
use crate::domain::{PrisonerAction, PrisonerCommit, PrisonerDomain, PrisonerId};

#[derive(Clone, Debug)]
pub struct PrisonerEnvState{
    previous_actions: Vec<PrisonerCommit>,
    last_actions: HashMap<PrisonerId, PrisonerAction>,
    reward_table: RewardTable
}
/*
impl State<PrisonerDomain> for PrisonerEnvState{
    fn update(&mut self, update: <PrisonerDomain as DomainParameters>::UpdateType)
        -> Result<(), <PrisonerDomain as DomainParameters>::GameErrorType> {
        //self.previous_actions.push()
        todo!()

    }
}

 */