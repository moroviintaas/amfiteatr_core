use std::fmt::{Display, Formatter, write};
use crate::env::EnvStateSequential;
use crate::domain::DomainParameters;
/*
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

 */


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvTrace<DP: DomainParameters, S: EnvStateSequential<DP>>{
    state_before: S,
    agent: DP::AgentId,
    //performed_action: CheckedAction<DP>,
    action: DP::ActionType,
    is_action_valid: bool
}

impl<DP: DomainParameters, S: EnvStateSequential<DP>> Display for EnvTrace<DP, S>
where S: Display, <DP as DomainParameters>::AgentId: Display,
      <DP as DomainParameters>::ActionType: Display{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ][ {} / {} ", self.state_before, self.agent, self.action)?;
        if !self.is_action_valid{
            write!(f, "!]")
        } else {
            write!(f, "]")
        }
    }
}


impl<DP: DomainParameters, S: EnvStateSequential<DP>> EnvTrace<DP, S>{

    pub fn new(state_before: S, agent: DP::AgentId,
               action: DP::ActionType, is_valid: bool) -> Self{
        /*let checked_action = match is_valid{
            false => CheckedAction::Invalid(action),
            true => CheckedAction::Valid(action)
        };

         */
        Self{state_before, agent, action, is_action_valid: is_valid}
    }

    pub fn state_before(&self) -> &S{
        &self.state_before
    }

    pub fn agent(&self) -> &DP::AgentId{
        &self.agent
    }

    pub fn action(&self) -> &DP::ActionType{
        &self.action
    }


    pub fn is_action_valid(&self) -> bool{
        self.is_action_valid
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameTrajectory<Tr>{
    history: Vec<Tr>,

}
pub type StdEnvTrajectory<DP, S> = GameTrajectory<EnvTrace<DP, S>>;
//DP: DomainParameters, S: EnvStateSequential<DP>
impl<Tr> GameTrajectory<Tr>{
    pub fn new() -> Self{
        Self{history: Vec::new()}
    }
    pub fn new_reserve(capacity: usize) -> Self{
        Self{history: Vec::with_capacity(capacity)}
    }
    pub fn list(&self) -> &Vec<Tr>{
        &self.history
    }
    pub fn push(&mut self, entry: Tr){
        self.history.push(entry);
    }
    pub fn clear(&mut self){
        self.history.clear()
    }
}

impl<DP: DomainParameters, S: EnvStateSequential<DP>> Default for GameTrajectory<EnvTrace<DP, S>>{
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