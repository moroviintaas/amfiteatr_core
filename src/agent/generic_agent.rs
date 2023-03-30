use std::marker::PhantomData;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent};
use crate::automatons::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::CommError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::state::agent::InformationSet;
use crate::state::State;

pub struct AgentGen<Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>{
    state: <P as Policy>::StateType,
    comm: Comm,
    policy: P,
    _phantom: PhantomData<Spec>,
}

impl <Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>
    AgentGen<Spec, P, Comm>{

    pub fn new(state: <P as Policy>::StateType, comm: Comm, policy: P) -> Self{
        Self{state, comm, policy,  _phantom:PhantomData::default()}
    }
}

impl<Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>
    CommunicatingAgent for AgentGen<Spec, P, Comm>
{
    type Outward = AgentMessage<Spec>;
    type Inward = EnvMessage<Spec>;
    type CommunicationError = CommError;

    fn send(&mut self, message: Self::Outward) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<Self::Inward, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>
    StatefulAgent for AgentGen<Spec, P, Comm>{
    type State = <P as Policy>::StateType;

    fn update(&mut self, state_update: <Self::State as State>::UpdateType) -> Result<(), <Self::State as State>::Error> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>
ActingAgent for AgentGen<Spec, P, Comm>{
    type Act = <<P as Policy>::StateType as InformationSet>::ActionType ;

    fn take_action(&self) -> Option<Self::Act> {
        self.policy.select_action(&self.state)
    }
}

