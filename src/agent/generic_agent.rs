use std::marker::PhantomData;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent};
use crate::agent::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::{DistinctAgent, PolicyAgent};
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::state::agent::InformationSet;
use crate::state::State;

pub struct AgentGen<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>{
    state: <P as Policy<Spec>>::StateType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<Spec>,
    id: Spec::AgentId
}

impl <Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    AgentGen<Spec, P, Comm>{

    pub fn new(id: Spec::AgentId, state: <P as Policy<Spec>>::StateType, comm: Comm, policy: P) -> Self{
        Self{state, comm, policy,  _phantom:PhantomData::default(), id}
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    CommunicatingAgent<Spec> for AgentGen<Spec, P, Comm>
{
    //type Outward = AgentMessage<Spec>;
    //type Inward = EnvMessage<Spec>;
    type CommunicationError = CommError<Spec>;

    fn send(&mut self, message: AgentMessage<Spec>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<Spec>, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
    StatefulAgent<Spec> for AgentGen<Spec, P, Comm>{
    type State = <P as Policy<Spec>>::StateType;

    fn update(&mut self, state_update: Spec::UpdateType) -> Result<(), Spec::GameErrorType> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
ActingAgent<Spec> for AgentGen<Spec, P, Comm>{

    fn take_action(&self) -> Option<Spec::ActionType> {
        self.policy.select_action(&self.state)
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
PolicyAgent<Spec> for AgentGen<Spec, P, Comm>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }
}

impl<Spec: ProtocolSpecification, P: Policy<Spec>,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError<Spec>>>
DistinctAgent<Spec> for AgentGen<Spec, P, Comm>{

    fn id(&self) -> &Spec::AgentId {
        &self.id
    }
}