


use crate::domain::{Construct, DomainParameters};
use crate::error::SztormError;

pub trait EnvironmentState<DP: DomainParameters>: Send{
    type Updates: IntoIterator<Item = (DP::AgentId, DP::UpdateType)>;

    fn current_player(&self) -> Option<DP::AgentId>;
    fn is_finished(&self) -> bool;

    fn forward(&mut self, agent: DP::AgentId, action: DP::ActionType)
        -> Result<Self::Updates, DP::GameErrorType>;

    //fn transform(&mut self, agent_id: &Spec::AgentId, action: Spec::ActionType) -> Result<Self::UpdatesCollection, Spec::GameErrorType>;

}

impl<DP: DomainParameters, T: EnvironmentState<DP>> EnvironmentState<DP> for Box<T>{
    type Updates = T::Updates;

    fn current_player(&self) -> Option<DP::AgentId> {
        self.as_ref().current_player()
    }

    fn is_finished(&self) -> bool {
        self.as_ref().is_finished()
    }

    fn forward(&mut self, agent: DP::AgentId, action: DP::ActionType) -> Result<Self::Updates, DP::GameErrorType> {
        self.as_mut().forward(agent, action)
    }
}


pub trait ConstructedEnvState<DP: DomainParameters, B>: EnvironmentState<DP> + Construct<B>{}
impl<DP: DomainParameters, B, T: EnvironmentState<DP> + Construct<B>> ConstructedEnvState<DP, B> for T{}


//impl<DP: DomainParameters, B, T: ConstructedEnvState<DP, B>> ConstructedEnvState<DP, B> for Box<T>{}



pub trait EnvironmentStateUniScore<DP: DomainParameters>: EnvironmentState<DP>{

    fn state_score_of_player(&self, agent: &DP::AgentId) -> DP::UniversalReward;

}

pub trait ExpandingState<DP: DomainParameters>{

    fn register_agent(&mut self, agent_id: DP::AgentId) -> Result<(), SztormError<DP>>;

}