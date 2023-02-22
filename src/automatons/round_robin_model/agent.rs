use std::marker::PhantomData;
use crate::agent::{AgentGen, AgentIdentifier, CommunicatingAgent, PolicyAgent, StatefulAgent};
use crate::automatons::AutomaticAgent;
use crate::automatons::policy::Policy;
use crate::comm::CommEndpoint;
use crate::error::{CommError, TurError};
use crate::error::ProtocolError::{NoPossibleAction, ReceivedKill};
use crate::error::TurError::ProtocolError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::state::agent::AgentState;
use crate::state::{State, StateUpdate};
use log::{info, warn, debug, error};
use crate::protocol::AgentMessage::NotifyError;
/*AgentState<ActionIteratorType=Spec::ActionIteratorType,
        ActionType=Spec::ActionType, Error=Spec::GameErrorType, UpdateType=Spec::UpdateType>,*/

/*
impl <Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=Spec::ActionType, InwardType=Spec::UpdateType, Error=CommError>>
    AutomaticAgent for AgentRR<Spec, P, Comm>{
    type ProtocolSpecType = Spec;

    fn run(&mut self) -> Result<(), TurError<Self::ProtocolSpecType>> {
        loop{

        }
    }
}

 */

pub trait AgentRR<Spec: ProtocolSpecification>{
    fn run_rr(&mut self) -> Result<(), TurError<Spec>>;
}

/*
impl <Spec: ProtocolSpecification, P: Policy,
    Comm: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>>
    AgentRR<Spec> for AgentGen<Spec, P, Comm>{
    fn run_rr(&mut self) -> Result<(), TurError<Spec>> {
        loop{
            match
        }
    }
}

*/

impl<Agnt, Spec > AgentRR<Spec> for Agnt
where Agnt: StatefulAgent + PolicyAgent<Act=Spec::ActionType> +
    CommunicatingAgent<Outward=AgentMessage<Spec>, Inward=EnvMessage<Spec>, CommunicationError=CommError>,
Spec: ProtocolSpecification<AgentId=<<Agnt as StatefulAgent>::State as AgentState>::Id>
{
    fn run_rr(&mut self) -> Result<(), TurError<Spec>> {
        loop{
            match self.recv(){
                Ok(message) => match message{
                    EnvMessage::YourMove => {
                        debug!("Agent {} received 'YourMove' signal.", self.state().id());
                        match self.select_action(){
                            None => {
                                error!("Agent {} has no possible action", self.state().id());
                                self.send(NotifyError(NoPossibleAction(*self.state().id()).into()))?;
                            }
                            Some(_) => {todo!()}
                        }
                    }
                    EnvMessage::GameFinished => {
                        info!("Agent {} received information that game is finished.", self.state().id());
                        return Ok(())

                    }
                    EnvMessage::Kill => {
                        info!("Agent {:?} received kill signal.", self.state().id());
                        return Err(ProtocolError(ReceivedKill(*self.state().id())))
                    }
                    EnvMessage::UpdateState(_) => {}
                    EnvMessage::ActionNotify(_) => {}
                    EnvMessage::ErrorNotify(_) => {}
                }
                Err(_) => {todo!()}
            }
        }
    }
}