use log::{error, info, warn};
use crate::env::{BroadcastingEnv, CommunicatingEnv, Environment, StatefulEnvironment};
use crate::error::{CommError, SztormError};
use crate::error::ProtocolError::PlayerExited;
use crate::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::protocol::EnvMessage::ErrorNotify;
use crate::state::env::EnvironmentState;
use crate::state::State;

pub trait EnvironmentRR<Spec: ProtocolSpecification>{
    fn env_run_rr(&mut self) -> Result<(), SztormError<Spec>>;
}

pub(crate) trait EnvironmentRRInternal<Spec: ProtocolSpecification>{
    fn notify_error(&mut self, error: SztormError<Spec>) -> Result<(), CommError>;
    fn send_message(&mut self, agent: &Spec::AgentId, message: EnvMessage<Spec>) -> Result<(), CommError>;
    fn process_action_and_inform(&mut self, player: Spec::AgentId, action: Spec::ActionType) -> Result<(), SztormError<Spec>>;

    //fn broadcast_message(&mut self ,message: EnvMessage<Spec>) -> Result<(), CommError>;
}

impl<'a, Env, Spec: ProtocolSpecification + 'a> EnvironmentRRInternal<Spec> for Env
where Env: CommunicatingEnv<AgentId = <<Env as StatefulEnvironment>::State as EnvironmentState>::AgentId,
    Outward=EnvMessage<Spec>, Inward=AgentMessage<Spec>, CommunicationError=CommError>
 + StatefulEnvironment + 'a
 + Environment<'a, Spec::AgentId>
 + BroadcastingEnv,
//<<Env as StatefulEnvironment>::State as State>::Error: Clone,
//TurError<Spec>: From<<<Env as StatefulEnvironment>::State as State>::Error>,
Spec: ProtocolSpecification<
    AgentId = <<Env as StatefulEnvironment>::State as EnvironmentState>::AgentId,
    UpdateType = <<Env as StatefulEnvironment>::State as State>::UpdateType,
    ActionType = <Env as StatefulEnvironment>::Act,
    GameErrorType = <<Env as StatefulEnvironment>::State as State>::Error>
 //Spec::AgentId =  <<Env as StatefulEnvironment>::State as EnvironmentState>::PlayerId
{
    fn notify_error(&mut self, error: SztormError<Spec>) -> Result<(), CommError> {
        self.send_to_all(ErrorNotify(error))
    }

    fn send_message(&mut self, agent: &Spec::AgentId, message: EnvMessage<Spec>) -> Result<(), CommError>{
        self.send_to(agent, message)
            .map_err(|e| {
                self.notify_error(e.clone().into())
                    .unwrap_or_else(|_| panic!("Failed broadcasting error message {}", &e));
                e
            })
    }

    fn process_action_and_inform(&mut self, player: Spec::AgentId, action: Spec::ActionType) -> Result<(), SztormError<Spec>> {
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
            Err(e) => {Err(SztormError::GameError(e))}
        }
    }


}


impl<'a, Env, Spec: ProtocolSpecification + 'a> EnvironmentRR<Spec> for Env
where Env: CommunicatingEnv<AgentId = <<Env as StatefulEnvironment>::State as EnvironmentState>::AgentId,
    Outward=EnvMessage<Spec>, Inward=AgentMessage<Spec>, CommunicationError=CommError>
 + StatefulEnvironment + 'a
 + Environment<'a, Spec::AgentId>
 + BroadcastingEnv,
//<<Env as StatefulEnvironment>::State as State>::Error: Clone,
//TurError<Spec>: From<<<Env as StatefulEnvironment>::State as State>::Error>,
Spec: ProtocolSpecification<
    AgentId = <<Env as StatefulEnvironment>::State as EnvironmentState>::AgentId,
    UpdateType = <<Env as StatefulEnvironment>::State as State>::UpdateType,
    ActionType = <Env as StatefulEnvironment>::Act,
    GameErrorType = <<Env as StatefulEnvironment>::State as State>::Error>
 //Spec::AgentId =  <<Env as StatefulEnvironment>::State as EnvironmentState>::PlayerId
{
    fn env_run_rr(&mut self) -> Result<(), SztormError<Spec>> {

        /*fn internal_error_notify(e: TurError<Spec>) -> Result<(), CommError>{
            self.send_to_all()
        }*/

        let first_player = match self.current_player(){
            None => {
                warn!("No first player, stopping environment.");
                return Ok(())
            }
            Some(n) => n
        };
        info!("Sending YourMove signal to first agent: {:?}", &first_player);
        self.send_to(&first_player, EnvMessage::YourMove)?;
        loop{
            for player in self.players(){
                match self.try_recv_from(player){
                    Ok(agent_message) => match agent_message{
                        AgentMessage::TakeAction(action) => {
                            info!("Player {} performs action: {:#}", &player, &action);
                            self.process_action_and_inform(*player, action)?;
                            if let Some(next_player) = self.current_player(){
                                self.send_message(&next_player, EnvMessage::YourMove)?;
                            }
                            if self.state().is_finished(){
                                info!("Game reached finished state");
                                self.send_to_all(EnvMessage::GameFinished)?;
                                return Ok(());

                            }
                            //match self.process_action(player, action.clone()){
                            //    Ok(iter) => {
                            //        for (ag, update) in iter{
                            //           /*self.send_to(&ag, EnvMessage::ActionNotify(AgentActionPair::new(player.clone(), action)))
                            //                .map_err(|e| {
                            //                    //self.send_to_all(ErrorNotify(TurError::CommError(e.clone())))
                            //                    self.notify_error(e.clone().into())
                            //                        .expect(&format!("Failed broadcasting error message {}", &e));
                            //                    //Err::<(), TurError<Spec>>(TurError::CommError(e))
                            //                    e
                            //                })?;
                            //             */
                            //            self.send_message(&ag, EnvMessage::ActionNotify(AgentActionPair::new(player.clone(), action.clone())))?;
                            //            self.send_message(&ag, EnvMessage::UpdateState(update))?;
                            //        }
                            //    }
                            //    Err(e) => {return Err(TurError::GameError(e))}
                            //}

                        }
                        AgentMessage::NotifyError(e) => {
                            error!("Player {} informed about error: {}", player, &e);
                            self.notify_error(e.clone())?;
                            return Err(e);
                        }
                        AgentMessage::Quit => {
                            error!("Player {} exited game.", player);
                            self.notify_error(SztormError::ProtocolError(PlayerExited(*player)))?;
                            return Err(SztormError::ProtocolError(PlayerExited(*player)))
                        }
                    },
                    Err(e) => match e{

                        CommError::TryRecvEmptyError | CommError::TryRecvDisconnectedError => {
                            //debug!("Empty channel");
                        },
                        err => {
                            error!("Failed trying to receive from {}", player);
                            self.send_to_all(EnvMessage::ErrorNotify(err.clone().into()))?;
                            return Err(SztormError::CommError(err));
                        }

                        /*error!("Failed trying to receive from {}", player);
                        self.send_to_all(EnvMessage::ErrorNotify(recv_error.clone().into()))?;
                        return Err(TurError::CommError(recv_error));*/
                    }
                }
            }
        }


    }
}

