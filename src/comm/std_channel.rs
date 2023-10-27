use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError, TryRecvError};
use crate::comm::endpoint::CommPort;
use crate::error::CommunicationError;
use crate::domain::{AgentMessage, EnvMessage};


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

