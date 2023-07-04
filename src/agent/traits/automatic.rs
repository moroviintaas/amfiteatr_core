use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent};
use crate::error::{CommError, SztormError};
use crate::error::ProtocolError::{NoPossibleAction, ReceivedKill};
use crate::error::SztormError::Protocol;
use crate::protocol::{AgentMessage, EnvMessage, DomainParameters};
use crate::state::agent::InformationSet;
use log::{info,  debug, error};
use crate::{DistinctAgent, PolicyAgent, RewardedAgent};
use crate::protocol::AgentMessage::{NotifyError, TakeAction};
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

pub trait AgentAuto<Spec: DomainParameters>: DistinctAgent<Spec>{
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

impl<Agnt, Spec > AgentAuto<Spec> for Agnt
where Agnt: StatefulAgent<Spec> + ActingAgent<Spec>
    + CommunicatingAgent<Spec, CommunicationError=CommError<Spec>>
    + PolicyAgent<Spec> + DistinctAgent<Spec>
    + RewardedAgent<Spec>,
      Spec: DomainParameters,
//<<Agnt as StatefulAgent>::State as State>::Error: Into<TurError<Spec>>
//SztormError<Spec>: From<<<Agnt as StatefulAgent<Spec>>::State as State>::Error>
{
    fn run_rr(&mut self) -> Result<(), SztormError<Spec>> {
        info!("Agent {} starts", self.state().id());
        let mut current_score = Spec::UniversalReward::default();
        loop{
            match self.recv(){
                Ok(message) => match message{
                    EnvMessage::YourMove => {
                        debug!("Agent {} received 'YourMove' signal.", self.state().id());
                        current_score = Default::default();

                        //debug!("Agent's {:?} possible actions: {:?}", self.state().id(), Vec::from_iter(self.state().available_actions().into_iter()));
                        debug!("Agent's {:?} possible actions: {}]", self.state().id(), self.state().available_actions().into_iter()
                            .fold(String::from("["), |a, b| a + &format!("{b:#}") + ", ").trim_end());
                        //match self.policy_select_action(){
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
                        self.finalize();
                        return Ok(())

                    }
                    EnvMessage::Kill => {
                        info!("Agent {:?} received kill signal.", self.state().id());
                        return Err(Protocol(ReceivedKill(*self.state().id())))
                    }
                    EnvMessage::UpdateState(su) => {
                        debug!("Agent {} received state update {:?}", self.state().id(), &su);
                        match self.update(su){
                            Ok(_) => {
                                debug!("Agent {:?}: successful state update", self.state().id());
                            }
                            Err(err) => {
                                error!("Agent error on updating state: {}", &err);
                                self.send(AgentMessage::NotifyError(SztormError::Game(err.clone())))?;
                                return Err(SztormError::Game(err));
                            }
                        }
                    }
                    EnvMessage::ActionNotify(a) => {
                        debug!("Agent {} received information that agent {} took action {:#}", self.state().id(), a.agent(), a.action());
                    }
                    EnvMessage::ErrorNotify(e) => {
                        error!("Agent {} received error notification {}", self.state().id(), &e)
                    }
                    EnvMessage::RewardFragment(r) =>{
                        //current_score = current_score + r;
                        //self.set_current_universal_reward(current_score.clone());
                        self.current_universal_reward_add(&r);
                    }
                }
                Err(e) => return Err(e.into())
            }
        }
    }
}