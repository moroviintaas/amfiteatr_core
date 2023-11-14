use std::sync::Mutex;
use crate::agent::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent, EnvRewardedAgent, Agent, InternalRewardedAgent, ScoringInformationSet, PresentPossibleActions};
use crate::error::{CommunicationError, AmfiError, WorldError};
use crate::error::ProtocolError::{NoPossibleAction, ReceivedKill};
use crate::error::AmfiError::Protocol;
use crate::domain::{AgentMessage, EnvMessage, DomainParameters};
use log::{info, debug, error, warn};
use crate::domain::AgentMessage::{NotifyError, TakeAction};

/// Trait for agents that perform their interactions with environment automatically,
/// without waiting for interrupting interaction from anyone but environment.
/// This trait describes behaviour of agent that is not necessary interested in
/// collecting rewards from environment.
/// Implementations are perfectly fine to skip messages about rewards coming
/// from environment. As trait suited for running game regarding collected rewards
/// refer to [`AutomaticAgentRewarded`](crate::agent::AutomaticAgentRewarded)
pub trait AutomaticAgent<Spec: DomainParameters>: Agent<Spec>{
    /// Runs agent beginning in it's current state (information set)
    /// and returns when game is finished.
    /// > __Note__ It is not specified how agent should react when encountering error.
    /// > One conception is to inform environment about error, which then should broadcast
    /// > error message to every agent and end game.
    fn run(&mut self) -> Result<(), AmfiError<Spec>>;
}

/// Trait for agents that perform their interactions with environment automatically,
/// without waiting for interrupting interaction from anyone but environment.
/// Difference between [`AutomaticAgent`](crate::agent::AutomaticAgent) is that
/// this method should collect rewards and somehow store rewards sent by environment.
pub trait AutomaticAgentRewarded<Spec: DomainParameters>: AutomaticAgent<Spec>{
    /// Runs agent beginning in it's current state (information set)
    /// and returns when game is finished.
    fn run_rewarded(&mut self) -> Result<(), AmfiError<Spec>>;
}



/// Generic implementation of AutomaticAgent - probably will be done via macro
/// in the future to avoid conflicts with custom implementations.
impl<Agnt, DP> AutomaticAgent<DP> for Agnt
where Agnt: StatefulAgent<DP> + ActingAgent<DP>
    + CommunicatingAgent<DP, CommunicationError=CommunicationError<DP>>
    + PolicyAgent<DP> + Agent<DP>
    + InternalRewardedAgent<DP>,
      DP: DomainParameters,
      <Agnt as StatefulAgent<DP>>::InfoSetType: ScoringInformationSet<DP> + PresentPossibleActions<DP>
{
    fn run(&mut self) -> Result<(), AmfiError<DP>> {
        info!("Agent {} starts", self.id());
        //let mut current_score = Spec::UniversalReward::default();
        loop{
            match self.recv(){
                Ok(message) => match message{
                    EnvMessage::YourMove => {
                        debug!("Agent {} received 'YourMove' signal.", self.id());
                        //current_score = Default::default();

                        //debug!("Agent's {:?} possible actions: {:?}", self.id(), Vec::from_iter(self.state().available_actions().into_iter()));
                        debug!("Agent's {} possible actions: {}]", self.id(), self.info_set().available_actions().into_iter()
                            .fold(String::from("["), |a, b| a + &format!("{b:#}") + ", ").trim_end());
                        //match self.policy_select_action(){
                        match self.take_action(){
                            None => {
                                error!("Agent {} has no possible action", self.id());
                                self.send(NotifyError(NoPossibleAction(self.id().clone()).into()))?;
                            }

                            Some(a) => {
                                info!("Agent {} selects action {:#}", self.id(), &a);
                                self.send(TakeAction(a))?;
                            }
                        }
                    }
                    EnvMessage::MoveRefused => {
                        self.add_explicit_subjective_score(
                            &<<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>
                            ::penalty_for_illegal())
                    }
                    EnvMessage::GameFinished => {
                        info!("Agent {} received information that game is finished.", self.id());
                        self.finalize();
                        return Ok(())

                    }
                    EnvMessage::GameFinishedWithIllegalAction(id) => {
                        warn!("Agent {} received information that game is finished with agent {id:} performing illegal action.", self.id());
                        self.finalize();
                        return Ok(())

                    }
                    EnvMessage::Kill => {
                        info!("Agent {:?} received kill signal.", self.id());
                        return Err(Protocol(ReceivedKill(self.id().clone())))
                    }
                    EnvMessage::UpdateState(su) => {
                        debug!("Agent {} received state update {:?}", self.id(), &su);
                        match self.update(su){
                            Ok(_) => {
                                debug!("Agent {:?}: successful state update", self.id());
                            }
                            Err(err) => {
                                error!("Agent {:?} error on updating state: {}", self.id(), &err);
                                self.send(AgentMessage::NotifyError(AmfiError::GameA(err.clone(), self.id().clone())))?;
                                return Err(AmfiError::GameA(err.clone(), self.id().clone()));
                            }
                        }
                    }
                    EnvMessage::ActionNotify(a) => {
                        debug!("Agent {} received information that agent {} took action {:#}", self.id(), a.agent(), a.action());
                    }
                    EnvMessage::ErrorNotify(e) => {
                        error!("Agent {} received error notification {}", self.id(), &e)
                    }
                    EnvMessage::RewardFragment(_r) =>{
                    }
                }
                Err(e) => return Err(e.into())
            }
        }
    }
}

impl<Agnt, DP> AutomaticAgentRewarded<DP> for Agnt
where Agnt: StatefulAgent<DP> + ActingAgent<DP>
    + CommunicatingAgent<DP, CommunicationError=CommunicationError<DP>>
    + PolicyAgent<DP> + Agent<DP>
    + EnvRewardedAgent<DP>
    + InternalRewardedAgent<DP>,
      DP: DomainParameters,
    <Agnt as StatefulAgent<DP>>::InfoSetType: ScoringInformationSet<DP> + PresentPossibleActions<DP>{
    fn run_rewarded(&mut self) -> Result<(), AmfiError<DP>>
    {
        info!("Agent {} starts", self.id());
        //let mut current_score = Spec::UniversalReward::default();
        loop{
            match self.recv(){
                Ok(message) => match message{
                    EnvMessage::YourMove => {
                        debug!("Agent {} received 'YourMove' signal.", self.id());
                        //current_score = Default::default();

                        //debug!("Agent's {:?} possible actions: {:?}", self.id(), Vec::from_iter(self.state().available_actions().into_iter()));
                        debug!("Agent's {:?} possible actions: {}]", self.id(), self.info_set().available_actions().into_iter()
                            .fold(String::from("["), |a, b| a + &format!("{b:#}") + ", ").trim_end());
                        //match self.policy_select_action(){
                        match self.take_action(){
                            None => {
                                error!("Agent {} has no possible action", self.id());
                                self.send(NotifyError(NoPossibleAction(self.id().clone()).into()))?;
                            }

                            Some(a) => {
                                info!("Agent {} selects action {:#}", self.id(), &a);
                                self.send(TakeAction(a))?;
                            }
                        }
                    }
                    EnvMessage::MoveRefused => {
                        self.add_explicit_subjective_score(
                            &<<Self as StatefulAgent<DP>>::InfoSetType as ScoringInformationSet<DP>>
                            ::penalty_for_illegal())
                    }
                    EnvMessage::GameFinished => {
                        info!("Agent {} received information that game is finished.", self.id());
                        self.finalize();
                        return Ok(())

                    }
                    EnvMessage::GameFinishedWithIllegalAction(id)=> {
                        warn!("Agent {} received information that game is finished with agent {id:} performing illegal action.", self.id());
                        self.finalize();
                        return Ok(())

                    }
                    EnvMessage::Kill => {
                        info!("Agent {:?} received kill signal.", self.id());
                        return Err(Protocol(ReceivedKill(self.id().clone())))
                    }
                    EnvMessage::UpdateState(su) => {
                        debug!("Agent {} received state update {:?}", self.id(), &su);
                        match self.update(su){
                            Ok(_) => {
                                debug!("Agent {:?}: successful state update", self.id());
                            }
                            Err(err) => {
                                error!("Agent {:?} error on updating state: {}", self.id(), &err);
                                self.send(AgentMessage::NotifyError(AmfiError::Game(err.clone())))?;
                                return Err(AmfiError::Game(err));
                            }
                        }
                    }
                    EnvMessage::ActionNotify(a) => {
                        debug!("Agent {} received information that agent {} took action {:#}", self.id(), a.agent(), a.action());
                    }
                    EnvMessage::ErrorNotify(e) => {
                        error!("Agent {} received error notification {}", self.id(), &e)
                    }
                    EnvMessage::RewardFragment(r) =>{
                        //current_score = current_score + r;
                        //self.set_current_universal_reward(current_score.clone());
                        debug!("Agent {} received reward fragment: {:?}", self.id(), r);
                        self.current_universal_reward_add(&r);
                    }
                }
                Err(e) => return Err(e.into())
            }
        }
    }
}
/*
impl<DP: DomainParameters, A: AutomaticAgent<DP>> AutomaticAgent<DP> for Mutex<A>{
    fn run(&mut self) -> Result<(), AmfiError<DP>> {
        let mut guard = self.lock().or_else(|_|Err(WorldError::<DP>::AgentMutexLock))?;
        let id = guard.id().clone();
        guard.run().map_err(|e|{
            error!("Agent {id:} encountered error: {e:}");
            e
        })
    }
}
*/
/*
impl<DP: DomainParameters, A: AutomaticAgent<DP>> AutomaticAgent<DP> for Box<A>{
    fn run(&mut self) -> Result<(), AmfiError<DP>> {
        self.run()
    }
}

 */