use crate::{InformationSet};
use crate::learning::GameTrace;
use crate::protocol::DomainParameters;

pub trait TracingAgent<DP: DomainParameters, S: InformationSet<DP>>{
    fn reset_trace(&mut self);
    //fn set_new_state(&mut self);
    fn game_trajectory(&self) -> &GameTrace<DP, S>;
    fn commit_trace(&mut self);

}