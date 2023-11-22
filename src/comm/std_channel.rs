use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError, TryRecvError};
use crate::comm::endpoint::CommPort;
use crate::error::CommunicationError;
use crate::domain::{AgentMessage, EnvMessage, DomainParameters};

use super::{AgentAdapter, EnvironmentAdapter, AgentCommEndpoint, EnvCommEndpoint};


#[derive(Debug)]
/*/// # Example:
/// ```
/// use std::thread::spawn;
/// use sztorm::{CommEndpoint};
/// use sztorm::error::CommError;
/// use sztorm::SyncComm;
/// let (mut com1, mut com2) = SyncComm::<String, String, CommError<Spec>>::new_pair();
/// let h1 = spawn(move || {
///     com1.send(format!("Hello")).unwrap();
/// });
/// let r = com2.recv().unwrap();
/// assert_eq!(r, format!("Hello"));
/// ```

 */
pub struct SyncComm<OT, IT, E: Error>{
    sender: Sender<OT>,
    receiver: Receiver<IT>,
    _phantom: PhantomData<E>
}


pub type SyncCommEnv<Spec> = SyncComm<EnvMessage<Spec>, AgentMessage<Spec>, CommunicationError<Spec>>;
pub type SyncCommAgent<Spec> = SyncComm<AgentMessage<Spec>, EnvMessage<Spec>,  CommunicationError<Spec>>;

impl<OT, IT, E: Error> SyncComm<OT, IT, E>
where SyncComm<OT, IT, E> :  CommPort<OutwardType = OT, InwardType = IT, Error = E>{
    pub fn new(sender: Sender<OT>, receiver: Receiver<IT>) -> Self{
        Self{sender, receiver, _phantom: PhantomData::default()}
    }
    pub fn new_pair() -> (Self, SyncComm<IT, OT, E>) {
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();

        (Self{sender: tx_1, receiver: rx_2, _phantom: PhantomData::default()},
        SyncComm{sender: tx_2, receiver: rx_1, _phantom: PhantomData::default()})
    }
    pub fn _decompose(self) -> (Sender<OT>, Receiver<IT>){
        (self.sender, self.receiver)
    }
}

impl<OT, IT, E> CommPort for SyncComm<OT, IT, E>
where E: Debug + Error + From<RecvError> + From<SendError<OT>> + From<TryRecvError> + From<SendError<IT>>,
OT: Debug, IT:Debug{
    type OutwardType = OT;

    type InwardType = IT;

    type Error = E;

    fn send(&mut self, message: OT) -> Result<(), E> {
        self.sender.send(message).map_err(|e| e.into())
    }

    fn receive_blocking(&mut self) -> Result<IT, E> {
        self.receiver.recv().map_err(|e| e.into())
    }

    fn receive_non_blocking(&mut self) -> Result<Option<IT>, E> {
        self.receiver.try_recv().map_or_else(
            |e| match e{
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => Err(e.into())
            },
            |message| Ok(Some(message))
        )

    }


}


pub enum DynComm<OT, IT, E: Error>{
    Std(SyncComm<OT, IT, E>),
    Dynamic(Box<dyn CommPort<OutwardType = OT, InwardType = IT, Error = E>>)
}

impl <OT: Debug, IT: Debug, E: Error> CommPort for DynComm<OT, IT, E>
where E: From<RecvError> + From<SendError<OT>> + From<TryRecvError> + From<SendError<IT>>{
    type OutwardType = OT;
    type InwardType = IT;
    type Error = E;

    fn send(&mut self, message: Self::OutwardType) -> Result<(), Self::Error> {
        match self{
            DynComm::Std(c) => c.send(message),
            DynComm::Dynamic(c) => {c.as_mut().send(message)}
        }
    }

    fn receive_blocking(&mut self) -> Result<Self::InwardType, Self::Error> {
        match self{
            DynComm::Std(c) => c.receive_blocking(),
            DynComm::Dynamic(c) => {c.as_mut().receive_blocking()}
        }
    }

    fn receive_non_blocking(&mut self) -> Result<Option<Self::InwardType>, Self::Error> {
        match self{
            DynComm::Std(c) => c.receive_non_blocking(),
            DynComm::Dynamic(c) => {c.as_mut().receive_non_blocking()}
        }
    }
}


pub struct AgentMpscPort<DP: DomainParameters>{
    id: DP::AgentId,
    sender: Sender<(DP::AgentId, AgentMessage<DP>)>,
    receiver: Receiver<EnvMessage<DP>>,
}

impl<DP: DomainParameters> AgentMpscPort<DP>{
    pub(crate) fn new(
        id: DP::AgentId, 
        sender: Sender<(DP::AgentId, AgentMessage<DP>)>,
        receiver: Receiver<EnvMessage<DP>> 
    ) -> Self{
        Self{id, sender, receiver}
    }
}

impl<DP: DomainParameters> AgentAdapter<DP> for AgentMpscPort<DP>{
    fn send(&mut self, message: AgentMessage<DP>) -> Result<(), CommunicationError<DP>> {
        self.sender.send((self.id.to_owned(), message)).map_err(|e| e.into())
    }

    fn receive(&mut self) -> Result<EnvMessage<DP>, CommunicationError<DP>> {
        self.receiver.recv().map_err(|e| e.into())
    }
}

pub struct EnvMpscPort<DP: DomainParameters>{
    sender_template: Sender<(DP::AgentId, AgentMessage<DP>)>,
    receiver: Receiver<(DP::AgentId, AgentMessage<DP>)>,
    senders: HashMap<DP::AgentId, Sender<EnvMessage<DP>>>
}

impl<DP: DomainParameters> EnvMpscPort<DP>{
    pub fn new() -> Self{
        let (sender_template, receiver) = channel();
        Self{receiver, sender_template, senders: HashMap::new()}
    }
    pub fn register_agent(&mut self, id: DP::AgentId) -> Result<AgentMpscPort<DP>, CommunicationError<DP>>{
        if self.senders.contains_key(&id){
            return Err(CommunicationError::DuplicatedAgent(id));
        } else {
            let (env_tx, agent_rx) = channel();
            let agent_adapter = AgentMpscPort::new(
                id.clone(),
                self.sender_template.clone(),
                agent_rx,
            );
            self.senders.insert(id, env_tx);
            Ok(agent_adapter)

        }

    }
}

impl<DP: DomainParameters> EnvironmentAdapter<DP> for EnvMpscPort<DP>{
    fn send(&mut self, agent: &<DP as DomainParameters>::AgentId, message: EnvMessage<DP>) 
    -> Result<(), CommunicationError<DP>> {
        let s = self.senders.get(agent)
            .ok_or_else(|| CommunicationError::ConnectionToAgentNotFound(agent.to_owned()))?;
        s.send(message).map_err(|e| e.into())
    }

    fn receive_blocking(&mut self) -> Result<(<DP as DomainParameters>::AgentId, AgentMessage<DP>), CommunicationError<DP>> {
        self.receiver.recv().map_err(|e| e.into())
    }

    fn receive_non_blocking(&mut self) -> Result<Option<(<DP as DomainParameters>::AgentId, AgentMessage<DP>)>, CommunicationError<DP>> {
        //self.receiver.try_recv().map_err(|e| e.into())
        self.receiver.try_recv().map_or_else(
            |e| match e{
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => Err(e.into())
            },
            |(id, message)| Ok(Some((id, message)))
        )
    }
    fn is_agent_connected(&self, agent_id: &DP::AgentId) -> bool{
        self.senders.contains_key(agent_id)
    }
}


pub struct EnvRRAdapter<DP: DomainParameters, T: EnvCommEndpoint<DP>>{
    endpoints: HashMap<DP::AgentId, T>,
}

impl <DP: DomainParameters, T: EnvCommEndpoint<DP>> EnvRRAdapter<DP, T>{

    pub fn new() -> Self{
        Self { endpoints: Default::default() }
    }

    pub fn register_agent(&mut self, agent_id: DP::AgentId, comm: T) -> Result<(), CommunicationError<DP>>{
        if self.endpoints.contains_key(&agent_id){
            return Err(CommunicationError::DuplicatedAgent(agent_id));
        } else {
            self.endpoints.insert(agent_id, comm);
            Ok(())
        }
    }
}

impl<DP: DomainParameters> EnvRRAdapter<DP, SyncCommEnv<DP>>{

    pub fn create_local_connection(&mut self, agent_id: DP::AgentId) -> Result<SyncCommAgent<DP>, CommunicationError<DP>>{
        if self.endpoints.contains_key(&agent_id){
            return Err(CommunicationError::DuplicatedAgent(agent_id));
        } else {
            let (env_comm, agent_comm) = SyncCommEnv::<DP>::new_pair();
            //let (tx_e, rx_a) = channel();
            //let (tx_a, rx_e) = channel();
            self.endpoints.insert(agent_id, env_comm);
            Ok(agent_comm)
        }
    }
}


impl<DP: DomainParameters> EnvRRAdapter<DP, Box<dyn EnvCommEndpoint<DP>>>{

    pub fn create_local_connection(&mut self, agent_id: DP::AgentId) -> Result<SyncCommAgent<DP>, CommunicationError<DP>>{
        if self.endpoints.contains_key(&agent_id){
            return Err(CommunicationError::DuplicatedAgent(agent_id));
        } else {
            let (env_comm, agent_comm) = SyncCommEnv::<DP>::new_pair();
            //let (tx_e, rx_a) = channel();
            //let (tx_a, rx_e) = channel();
            self.endpoints.insert(agent_id, Box::new(env_comm));
            Ok(agent_comm)
        }
    }
}


impl <DP: DomainParameters, T: EnvCommEndpoint<DP>> EnvironmentAdapter<DP> for EnvRRAdapter<DP, T>{
    fn send(&mut self, agent: &<DP as DomainParameters>::AgentId, message: EnvMessage<DP>) 
    -> Result<(), CommunicationError<DP>> {
        /*self.endpoints.get_mut(agent)
            .ok_or_else(|| CommunicationError::ConnectionToAgentNotFound(agent.to_owned()))?
            .send(message)*/
        if let Some(s) = self.endpoints.get_mut(agent){
            s.send(message)
        } else {
            Err(CommunicationError::ConnectionToAgentNotFound(agent.to_owned()))
        }
        
    }

    fn receive_blocking(&mut self) -> Result<(<DP as DomainParameters>::AgentId, AgentMessage<DP>), CommunicationError<DP>> {
        loop{
            for (agent, endpoint) in self.endpoints.iter_mut(){
                match endpoint.receive_non_blocking(){
                    Ok(None) => {},
                    Ok(Some(message)) => {
                        return Ok((agent.to_owned(), message));
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn receive_non_blocking(&mut self) -> Result<Option<(<DP as DomainParameters>::AgentId, AgentMessage<DP>)>, CommunicationError<DP>> {
        for (agent, endpoint) in self.endpoints.iter_mut(){
            match endpoint.receive_non_blocking(){
                Ok(None) => {},
                Ok(Some(message)) => {
                    return Ok(Some((agent.to_owned(), message)))
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
        return Ok(None);

    }

    fn is_agent_connected(&self, agent_id: &<DP as DomainParameters>::AgentId) -> bool {
        self.endpoints.contains_key(agent_id)
    }
}