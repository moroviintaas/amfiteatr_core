use std::collections::HashMap;
use log::{warn, info, error};

use crate::{error::{AmfiError, CommunicationError}, domain::{DomainParameters, EnvMessage, AgentMessage}, env::EnvStateSequential};
use crate::agent::ListPlayers;
use crate::domain::Reward;
use crate::env::ScoreEnvironment;
use crate::error::AmfiError::GameA;

use super::{StatefulEnvironment, ConnectedEnvironment, BroadConnectedEnvironment};
use crate::error::ProtocolError::PlayerExited;

pub trait AutoEnvironment<DP: DomainParameters>{
    fn run(&mut self) -> Result<(), AmfiError<DP>>;
}

pub trait AutoEnvironmentWithScores<DP: DomainParameters>{
    fn run_with_scores(&mut self) -> Result<(), AmfiError<DP>>;
    //fn run_with_scores_and_penalties<P: Fn(&DP::AgentId) -> DP::UniversalReward>(&mut self, penalty: P) -> Result<(), AmfiError<DP>>;
}

pub trait AutoEnvironmentWithScoresAndPenalties<DP: DomainParameters>{
    fn run_with_scores_and_penalties<P: Fn(&DP::AgentId) -> DP::UniversalReward>(&mut self, penalty: P) -> Result<(), AmfiError<DP>>;
}


pub(crate) trait AutoEnvInternals<DP: DomainParameters>{
    fn notify_error(&mut self, error: AmfiError<DP>) -> Result<(), CommunicationError<DP>>;
    fn send_message(&mut self, agent: &DP::AgentId, message: EnvMessage<DP>) -> Result<(), CommunicationError<DP>>;
    fn process_action_and_inform(&mut self, player: DP::AgentId, action: &DP::ActionType) -> Result<(), AmfiError<DP>>;
}

impl <
    DP: DomainParameters,
    E: StatefulEnvironment<DP> 
        + ConnectedEnvironment<DP> 
        + BroadConnectedEnvironment<DP>
> AutoEnvInternals<DP> for E{
    fn notify_error(&mut self, error: AmfiError<DP>) -> Result<(), CommunicationError<DP>> {
        self.send_all(EnvMessage::ErrorNotify(error))
    }

    fn send_message(&mut self, agent: &<DP as DomainParameters>::AgentId, message: EnvMessage<DP>) -> Result<(), CommunicationError<DP>> {
        self.send(agent, message)
            .map_err(|e|{
                self.notify_error(e.clone().into())
                    .unwrap_or_else(|_|panic!("Failed broadcasting error message {}", &e));
                e
            })
    }

    fn process_action_and_inform(&mut self, player: <DP as DomainParameters>::AgentId, action: &<DP as DomainParameters>::ActionType) -> Result<(), AmfiError<DP>> {
        match self.process_action(&player, action){
            Ok(iter) => {
                for (ag, update) in iter{
                    self.send_message(&ag, EnvMessage::UpdateState(update))?;
                }
                Ok(())
            }
            Err(e) => {Err(AmfiError::Game(e))}
        }
    }
}

impl <
    DP: DomainParameters,
    E: ScoreEnvironment<DP>
        + ConnectedEnvironment<DP> 
        + BroadConnectedEnvironment<DP>
> AutoEnvironment<DP> for E{
    fn run(&mut self) -> Result<(), AmfiError<DP>> {

        let first_player = match self.current_player(){
            None => {
                warn!("No first player, stopping environment.");
                return Ok(())
            }
            Some(n) => n
        };
        info!("Sending YourMove signal to first agent: {:?}", &first_player);
        self.send(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            match self.receive_blocking(){
                Ok((player, message)) => {
                    match message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);

                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e| {
                                                let _ = self.send_all(EnvMessage::ErrorNotify(e.clone().into()));
                                                e
                                            })?;

                                    }

                                }
                                Err(e) => {
                                    error!("Action was refused or caused error in updating state: {e:}");
                                    let _ = self.send(&player, EnvMessage::MoveRefused);
                                    let _ = self.send_all(EnvMessage::GameFinishedWithIllegalAction(player.clone()));
                                    return Err(AmfiError::GameA(e, player));
                                }
                            }
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_all(EnvMessage::ErrorNotify(er.clone().into()));
                                        er

                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        },
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player.clone())))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    }
                }
                Err(e) => match e{

                    CommunicationError::RecvEmptyBufferError(_) | CommunicationError::RecvPeerDisconnectedError(_) |
                    CommunicationError::RecvEmptyBufferErrorUnspecified | CommunicationError::RecvPeerDisconnectedErrorUnspecified => {
                        //debug!("Empty channel");
                    },
                    err => {
                        error!("Failed trying to receive message");
                        self.send_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                        return Err(AmfiError::Communication(err));
                    }


                }
            }
            
        }   
    }
}


impl <
    DP: DomainParameters,
    E: ScoreEnvironment<DP>
        + ConnectedEnvironment<DP>
        + BroadConnectedEnvironment<DP>
        + ListPlayers<DP>
> AutoEnvironmentWithScores<DP> for E{
    fn run_with_scores(&mut self) -> Result<(), AmfiError<DP>> {
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
        self.send(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            match self.receive_blocking(){
                Ok((player, message)) => {
                    match message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);

                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e| {
                                                let _ = self.send_all(EnvMessage::ErrorNotify(e.clone().into()));
                                                e
                                            })?;

                                    }
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        self.send(player, EnvMessage::RewardFragment(reward))?;
                                    }

                                }
                                Err(e) => {
                                    error!("Action was refused or caused error in updating state: {e:}");
                                    let _ = self.send(&player, EnvMessage::MoveRefused);
                                    let _ = self.send_all(EnvMessage::GameFinishedWithIllegalAction(player.clone()));
                                    return Err(AmfiError::GameA(e, player));
                                }
                            }
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_all(EnvMessage::ErrorNotify(er.clone().into()));
                                        er

                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        },
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player.clone())))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    }
                }
                Err(e) => match e{

                    CommunicationError::RecvEmptyBufferError(_) | CommunicationError::RecvPeerDisconnectedError(_) |
                    CommunicationError::RecvEmptyBufferErrorUnspecified | CommunicationError::RecvPeerDisconnectedErrorUnspecified => {
                        //debug!("Empty channel");
                    },
                    err => {
                        error!("Failed trying to receive message");
                        self.send_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                        return Err(AmfiError::Communication(err));
                    }


                }
            }

        }
    }


}

impl <
    DP: DomainParameters,
    E: ScoreEnvironment<DP>
        + ConnectedEnvironment<DP>
        + BroadConnectedEnvironment<DP>
        + ListPlayers<DP>
> AutoEnvironmentWithScoresAndPenalties<DP> for E{
    fn run_with_scores_and_penalties<P: Fn(&DP::AgentId) -> DP::UniversalReward>(&mut self, penalty: P) -> Result<(), AmfiError<DP>> {
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
        self.send(&first_player, EnvMessage::YourMove).map_err(|e|e.specify_id(first_player))?;
        loop{
            match self.receive_blocking(){
                Ok((player, message)) => {
                    match message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);

                            match self.process_action(&player, &action){
                                Ok(updates) => {
                                    for (ag, update) in updates{
                                        self.send_message(&ag, EnvMessage::UpdateState(update))
                                            .map_err(|e| {
                                                let _ = self.send_all(EnvMessage::ErrorNotify(e.clone().into()));
                                                e
                                            })?;

                                    }
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        self.send(player, EnvMessage::RewardFragment(reward))?;
                                    }

                                }
                                Err(e) => {
                                    error!("Player {player:} performed illegal action: {action:}");
                                    let _ = self.send(&player, EnvMessage::MoveRefused);
                                    let _ = self.send(&player, EnvMessage::RewardFragment(penalty(&player)));
                                    for (player, score) in actual_universal_scores.iter_mut(){

                                        let reward = self.actual_score_of_player(player) - score.clone();
                                        *score = self.actual_score_of_player(player);
                                        let _ = self.send(player, EnvMessage::RewardFragment(reward));
                                    }
                                    let _ = self.send_all(EnvMessage::GameFinishedWithIllegalAction(player.clone()));
                                    return Err(GameA(e, player));
                                }
                            }
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)
                                    .map_err(|e| {
                                        let er = e.specify_id(next_player);
                                        let _ = self.send_all(EnvMessage::ErrorNotify(er.clone().into()));
                                        er

                                    })?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }


                        },
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(AmfiError::Protocol(PlayerExited(player.clone())))?;
                            return Err(AmfiError::Protocol(PlayerExited(player)))
                        }
                    }
                }
                Err(e) => match e{

                    CommunicationError::RecvEmptyBufferError(_) | CommunicationError::RecvPeerDisconnectedError(_) |
                    CommunicationError::RecvEmptyBufferErrorUnspecified | CommunicationError::RecvPeerDisconnectedErrorUnspecified => {
                        //debug!("Empty channel");
                    },
                    err => {
                        error!("Failed trying to receive message");
                        self.send_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                        return Err(AmfiError::Communication(err));
                    }


                }
            }

        }
    }

}