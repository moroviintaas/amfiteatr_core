use std::fmt::{Display, Formatter};
use crate::env::EnvironmentState;
use crate::protocol::DomainParameters;

#[derive(Clone, Debug)]
pub enum CheckedAction<DP: DomainParameters>{
    Valid(DP::ActionType),
    Invalid(DP::ActionType)
}

impl <DP: DomainParameters> Display for CheckedAction<DP>
where <DP as DomainParameters>::ActionType: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            CheckedAction::Valid(a) => write!(f, "{a:}"),
            CheckedAction::Invalid(a) => write!(f, "(INVALID){a:}")
        }
    }
}

impl<DP: DomainParameters> CheckedAction<DP>{
    pub fn action(&self) -> &DP::ActionType{
        match &self{
            CheckedAction::Valid(a) => a,
            CheckedAction::Invalid(a) => a
        }
    }
    pub fn take(self) -> DP::ActionType{
        match self{
            CheckedAction::Valid(a) => {a}
            CheckedAction::Invalid(a) => {a}
        }
    }

    pub fn is_valid(&self) -> bool{
        match self{
            CheckedAction::Valid(_) => true,
            CheckedAction::Invalid(_) => false
        }
    }
}



#[derive(Debug, Clone)]
pub struct EnvTrace<DP: DomainParameters, S: EnvironmentState<DP>>{
    state_before: S,
    agent: DP::AgentId,
    performed_action: CheckedAction<DP>,
}

impl<DP: DomainParameters, S: EnvironmentState<DP>> Display for EnvTrace<DP, S>
where S: Display, <DP as DomainParameters>::AgentId: Display,
      <DP as DomainParameters>::ActionType: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ][ {} / {} ]", self.state_before, self.agent, self.performed_action)
    }
}


impl<DP: DomainParameters, S: EnvironmentState<DP>> EnvTrace<DP, S>{

    pub fn new(state_before: S, agent: DP::AgentId,
               action: DP::ActionType, is_valid: bool) -> Self{
        let checked_action = match is_valid{
            false => CheckedAction::Invalid(action),
            true => CheckedAction::Valid(action)
        };
        Self{state_before, agent, performed_action: checked_action}
    }

    pub fn state_before(&self) -> &S{
        &self.state_before
    }

    pub fn agent(&self) -> &DP::AgentId{
        &self.agent
    }

    pub fn action(&self) -> &DP::ActionType{
        self.performed_action.action()
    }


    pub fn is_action_valid(&self) -> bool{
        self.performed_action.is_valid()
    }
}

#[derive(Debug, Clone)]
pub struct EnvTrajectory<DP: DomainParameters, S: EnvironmentState<DP>>{
    history: Vec<EnvTrace<DP, S>>,

}

impl<DP: DomainParameters, S: EnvironmentState<DP>> EnvTrajectory<DP, S>{
    pub fn new() -> Self{
        Self{history: Vec::new()}
    }
    pub fn new_reserve(capacity: usize) -> Self{
        Self{history: Vec::with_capacity(capacity)}
    }
    pub fn list(&self) -> &Vec<EnvTrace<DP, S>>{
        &self.history
    }
    pub fn push(&mut self, entry: EnvTrace<DP, S>){
        self.history.push(entry);
    }
    pub fn clear(&mut self){
        self.history.clear()
    }
}

impl<DP: DomainParameters, S: EnvironmentState<DP>> Default for EnvTrajectory<DP, S>{
    fn default() -> Self {
        Self{history: Default::default()}
    }
}

/*
impl<'a, DP: DomainParameters, S: EnvironmentState<DP>> IntoIterator for &'a EnvHistory<DP, S>{
    type Item = &'a HistoryEntry<DP, S>;
    type IntoIter = <Vec<HistoryEntry<DP, S>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.history.into_iter()
    }
}

 */