use crate::domain::DomainParameters;

pub trait IdAgent<DP: DomainParameters>{
    fn id(&self) -> &<DP as DomainParameters>::AgentId;
}