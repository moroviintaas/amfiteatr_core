use crate::error::SztormError;
use crate::protocol::DomainParameters;

pub trait ExpandingState<DP: DomainParameters>{

    fn register_agent(&mut self, agent_id: DP::AgentId) -> Result<(), SztormError<DP>>;

}