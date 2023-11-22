use crate::domain::DomainParameters;


pub trait ListPlayers<DP: DomainParameters>{
    type IterType: Iterator<Item = DP::AgentId>;

    fn players(&self) -> Self::IterType;
}
/*
pub trait ListRefPlayers<DP: DomainParameters>{
    
    fn players_ref(&self) -> &[DP::AgentId];
    
}
*/
/* 
pub trait ListRefPlayers<'a, DP: DomainParameters>: 'a{
    type IterRefType: Iterator<Item = &'a DP::AgentId>;

    fn players_ref(&self) -> Self::IterRefType;
    
}
*/