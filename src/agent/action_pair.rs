use std::fmt::{Display, Formatter};
use crate::action::Action;
use crate::agent::AgentIdentifier;
use crate::state::StateUpdate;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub struct AgentActionPair<Agt: AgentIdentifier, Act: Action>{
    action: Act,
    agent: Agt
}

impl<Agt: AgentIdentifier, Act: Action> AgentActionPair<Agt, Act>{
    pub fn new(agent_id: Agt, action: Act) -> Self { Self{action, agent: agent_id}}

    pub fn action(&self) -> &Act { &self.action}
    pub fn agent(&self) -> &Agt {&self.agent}
}

impl<Agt: AgentIdentifier, Act: Action> Display for AgentActionPair<Agt, Act> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Update [agent: {} performed action {}", self.agent, self.action)
    }
}

impl<Agt: AgentIdentifier, Act: Action> StateUpdate for AgentActionPair<Agt, Act>{

}