use crate::domain::DomainParameters;

pub trait AgentWithId<DP: DomainParameters>{
    fn id(&self) -> &<DP as DomainParameters>::AgentId;
}