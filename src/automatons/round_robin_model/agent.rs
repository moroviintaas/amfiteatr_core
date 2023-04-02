use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent};
use crate::error::{CommError, SztormError};
use crate::error::ProtocolError::{NoPossibleAction, ReceivedKill};
use crate::error::SztormError::ProtocolError;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::state::agent::InformationSet;
use log::{info,  debug, error};
use crate::protocol::AgentMessage::{NotifyError, TakeAction};
use crate::state::State;
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
    fn run_rr(&mut self) -> Result<(), SztormError<Spec>>;
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
where Agnt: StatefulAgent+ ActingAgent<Act=Spec::ActionType> +
    CommunicatingAgent<Outward=AgentMessage<Spec>, Inward=EnvMessage<Spec>, CommunicationError=CommError>,
      Spec: ProtocolSpecification<
    AgentId=<<Agnt as StatefulAgent>::State as InformationSet>::Id,
    UpdateType=<<Agnt as StatefulAgent>::State as State>::UpdateType,
    GameErrorType=<<Agnt as StatefulAgent>::State as State>::Error>,
//<<Agnt as StatefulAgent>::State as State>::Error: Into<TurError<Spec>>
SztormError<Spec>: From<<<Agnt as StatefulAgent>::State as State>::Error>
{
    fn run_rr(&mut self) -> Result<(), SztormError<Spec>> {

        loop{
            match self.recv(){
                Ok(message) => match message{
                    EnvMessage::YourMove => {
                        debug!("Agent {} received 'YourMove' signal.", self.state().id());

                        //debug!("Agent's {:?} possible actions: {:?}", self.state().id(), Vec::from_iter(self.state().available_actions().into_iter()));
                        debug!("Agent's {:?} possible actions: {}]", self.state().id(), self.state().available_actions().into_iter()
                            .fold(String::from("["), |a, b| a + &format!("{b:#}") + ", ").trim_end());
                        match self.take_action(){
                            None => {
                                error!("Agent {} has no possible action", self.state().id());
                                self.send(NotifyError(NoPossibleAction(*self.state().id()).into()))?;
                            }

                            Some(a) => {
                                info!("Agent {} selects action {:#}", self.state().id(), &a);
                                self.send(TakeAction(a))?;
                            }
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
                    EnvMessage::UpdateState(su) => {
                        debug!("Agent {} received state update {:?}", self.state().id(), &su);
                        match self.update(su){
                            Ok(_) => {
                                debug!("Agent {:?}: successful state update", self.state().id());
                            }
                            Err(err) => {
                                error!("Agent error on updating state: {}", &err);
                                self.send(AgentMessage::NotifyError(SztormError::GameError(err.clone())))?;
                                return Err(SztormError::GameError(err));
                            }
                        }
                    }
                    EnvMessage::ActionNotify(a) => {
                        debug!("Agent {} received information that agent {} took action {:#}", self.state().id(), a.agent(), a.action());
                    }
                    EnvMessage::ErrorNotify(e) => {
                        error!("Agent {} received error notification {}", self.state().id(), &e)
                    }
                }
                Err(e) => return Err(e.into())
            }
        }
    }
}