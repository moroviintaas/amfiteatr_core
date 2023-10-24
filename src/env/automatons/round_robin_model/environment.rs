use std::collections::HashMap;
use log::{debug, error, info, warn};
use crate::env::{BroadcastingEnv, CommunicatingEnv, EnvStateSequential, EnvironmentWithAgents, ScoreEnvironment, StatefulEnvironment};
use crate::error::{CommError, AmfiError};
use crate::error::ProtocolError::PlayerExited;
use crate::domain::{AgentMessage, EnvMessage, DomainParameters, Reward};
use crate::domain::EnvMessage::ErrorNotify;
use crate::error::AmfiError::GameA;


pub trait RoundRobinEnvironment<DP: DomainParameters>{
    fn run_round_robin(&mut self) -> Result<(), AmfiError<DP>>;
}
pub trait RoundRobinUniversalEnvironment<DP: DomainParameters> : RoundRobinEnvironment<DP>{
    fn run_round_robin_uni_rewards(&mut self) -> Result<(), AmfiError<DP>>;
}
pub trait RoundRobinPenalisingUniversalEnvironment<DP: DomainParameters>: RoundRobinUniversalEnvironment<DP>{
    fn run_round_robin_uni_rewards_penalise(&mut self, penalty: DP::UniversalReward) -> Result<(), AmfiError<DP>>;
}



pub(crate) trait EnvironmentRRInternal<DP: DomainParameters>{
    fn notify_error(&mut self, error: AmfiError<DP>) -> Result<(), CommError<DP>>;
    fn send_message(&mut self, agent: &DP::AgentId, message: EnvMessage<DP>) -> Result<(), CommError<DP>>;
    fn process_action_and_inform(&mut self, player: DP::AgentId, action: &DP::ActionType) -> Result<(), AmfiError<DP>>;

    //fn broadcast_message(&mut self ,message: EnvMessage<Spec>) -> Result<(), CommError>;
}

impl<'a, Env, DP: DomainParameters + 'a> EnvironmentRRInternal<DP> for Env
where Env: CommunicatingEnv<DP, CommunicationError=CommError<DP>>
 + StatefulEnvironment<DP> + 'a
 + EnvironmentWithAgents<DP>
 + BroadcastingEnv<DP>,

DP: DomainParameters
{
    fn notify_error(&mut self, error: AmfiError<DP>) -> Result<(), CommError<DP>> {
        self.send_to_all(ErrorNotify(error))
    }

    fn send_message(&mut self, agent: &DP::AgentId, message: EnvMessage<DP>) -> Result<(), CommError<DP>>{
        self.send_to(agent, message)
            .map_err(|e| {
                self.notify_error(e.clone().into())
                    .unwrap_or_else(|_| panic!("Failed broadcasting error message {}", &e));
                e
            })
    }

    fn process_action_and_inform(&mut self, player: DP::AgentId, action: &DP::ActionType) -> Result<(), AmfiError<DP>> {
        match self.process_action(&player, action){
            Ok(iter) => {
                //let mut n=0;
                for (ag, update) in iter{
                    //debug!("{}", n);
                    //n+= 1;
                    //self.send_message(&ag, EnvMessage::ActionNotify(AgentActionPair::new(player.clone(), action.clone())))?;
                    self.send_message(&ag, EnvMessage::UpdateState(update))?;
                }
                Ok(())
            }
            Err(e) => {Err(AmfiError::Game(e))}
        }
    }


}


impl<'a, Env, DP: DomainParameters + 'a> RoundRobinEnvironment<DP> for Env
where Env: CommunicatingEnv<DP, CommunicationError=CommError<DP>>
 + StatefulEnvironment<DP> + 'a
 + EnvironmentWithAgents<DP>
 + BroadcastingEnv<DP>, DP: DomainParameters {
    fn run_round_robin(&mut self) -> Result<(), AmfiError<DP>> {
        let first_player = match self.current_player(){
            None => {
                warn!("No first player, stopping environment.");
                return Ok(())
            }
            Some(n) => n
        };
        info!("Sending YourMove signal to first agent: {:?}", &first_player);
        self.send_to(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            for player in self.players(){
                match self.try_recv_from(&player){
                    Ok(Some(agent_message)) => match agent_message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);

                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e| {
                                                let _ = self.send_to_all(ErrorNotify(e.clone().into()));
                                                e
                                            })?;

                                    }
                                }
                                Err(e) => {
                                    error!("Action was refused or caused error in updating state: {e:}");
                                    let _ = self.send_to(&player, EnvMessage::MoveRefused);
                                    let _ = self.send_to_all(EnvMessage::GameFinishedWithIllegalAction(player));
                                    return Err(GameA(e, player));
                                }
                            }
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_to_all(ErrorNotify(er.clone().into()));
                                        er

                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_to_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        }
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player)))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    },
                    Ok(None) => {},
                    Err(e) => match e{

                        CommError::RecvEmptyBufferError(_) | CommError::RecvPeerDisconnectedError(_) |
                        CommError::RecvEmptyBufferErrorUnspecified | CommError::RecvPeerDisconnectedErrorUnspecified => {
                            //debug!("Empty channel");
                        },
                        err => {
                            error!("Failed trying to receive from {}", player);
                            self.send_to_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                            return Err(AmfiError::Comm(err));
                        }


                    }
                }
            }
        }
    }
}

impl<'a, Env, DP: DomainParameters + 'a> RoundRobinUniversalEnvironment<DP> for Env
where Env: CommunicatingEnv<DP, CommunicationError=CommError<DP>>
 + ScoreEnvironment<DP> + 'a
 + EnvironmentWithAgents<DP>
 + BroadcastingEnv<DP>, DP: DomainParameters {
    fn run_round_robin_uni_rewards(&mut self) -> Result<(), AmfiError<DP>> {
        let mut actual_universal_scores: HashMap<DP::AgentId, DP::UniversalReward> = self.players().into_iter()
            .map(|id|{
                (id, DP::UniversalReward::neutral())
            }).collect();
        let first_player = match self.current_player(){
            None => {
                warn!("No first player, stopping environment.");
                return Ok(())
            }
            Some(n) => n
        };
        info!("Sending YourMove signal to first agent: {:?}", &first_player);
        self.send_to(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            for player in self.players(){
                match self.try_recv_from(&player){
                    Ok(Some(agent_message)) => match agent_message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);

                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e|{
                                                let _ = self.send_to_all(ErrorNotify(e.clone().into()));
                                                e
                                            })?;
                                    }
                                    debug!("Preparing rewards, previous scores: {:?}", actual_universal_scores);
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        self.send_to(player, EnvMessage::RewardFragment(reward))?;
                                    }
                                }
                                Err(e) => {
                                    error!("Action was refused or caused error in updating state: {e:}");
                                    let _ = self.send_to(&player, EnvMessage::MoveRefused);

                                    let _ = self.send_to_all(EnvMessage::GameFinishedWithIllegalAction(player));
                                    return Err(GameA(e, player));
                                }
                            }


                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_to_all(ErrorNotify(er.clone().into()));
                                        er
                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_to_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        }
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player)))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    },
                    Ok(None) => {},
                    Err(e) => match e{

                        CommError::RecvEmptyBufferError(_) | CommError::RecvPeerDisconnectedError(_) |
                        CommError::RecvEmptyBufferErrorUnspecified | CommError::RecvPeerDisconnectedErrorUnspecified => {
                            //debug!("Empty channel");
                        },
                        err => {
                            error!("Failed trying to receive from {}", player);
                            self.send_to_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                            return Err(AmfiError::Comm(err));
                        }


                    }
                }
            }
        }
    }
}

impl<'a, Env, DP: DomainParameters + 'a> RoundRobinPenalisingUniversalEnvironment<DP> for Env
where Env: CommunicatingEnv<DP, CommunicationError=CommError<DP>>
 + ScoreEnvironment<DP> + 'a
 + EnvironmentWithAgents<DP>
 + BroadcastingEnv<DP>, DP: DomainParameters{
    fn run_round_robin_uni_rewards_penalise(&mut self, penalty: DP::UniversalReward) -> Result<(), AmfiError<DP>> {
        let mut actual_universal_scores: HashMap<DP::AgentId, DP::UniversalReward> = self.players().into_iter()
            .map(|id|{
                (id, DP::UniversalReward::neutral())
            }).collect();
        let first_player = match self.current_player(){
            None => {
                warn!("No first player, stopping environment.");
                return Ok(())
            }
            Some(n) => n
        };
        info!("Sending YourMove signal to first agent: {:?}", &first_player);
        self.send_to(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            for player in self.players(){
                match self.try_recv_from(&player){
                    Ok(Some(agent_message)) => match agent_message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);
                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e|{
                                                let _ = self.send_to_all(ErrorNotify(e.clone().into()));
                                                e
                                            })?;
                                    }
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        self.send_to(player, EnvMessage::RewardFragment(reward))?;
                                    }
                                }
                                Err(e) => {
                                    error!("Player {player:} performed illegal action: {action:}");
                                    let _ = self.send_to(&player, EnvMessage::MoveRefused);
                                    let _ = self.send_to(&player, EnvMessage::RewardFragment(penalty));
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        let _ = self.send_to(player, EnvMessage::RewardFragment(reward));
                                    }
                                    let _ = self.send_to_all(EnvMessage::GameFinishedWithIllegalAction(player));
                                    return Err(GameA(e, player));
                                }
                            }
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_to_all(ErrorNotify(er.clone().into()));
                                        er
                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_to_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        }
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player)))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    },
                    Ok(None) => {},
                    Err(e) => match e{

                        CommError::RecvEmptyBufferError(_) | CommError::RecvPeerDisconnectedError(_) |
                        CommError::RecvEmptyBufferErrorUnspecified | CommError::RecvPeerDisconnectedErrorUnspecified => {
                            //debug!("Empty channel");
                        },
                        err => {
                            error!("Failed trying to receive from {}", player);
                            self.send_to_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                            return Err(AmfiError::Comm(err));
                        }


                    }
                }
            }
        }
    }
}

