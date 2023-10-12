use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SyncSender, SendError, TryRecvError};
use crate::comm::endpoint::CommEndpoint;
use crate::error::CommError;
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
    sender: SyncSender<OT>,
    receiver: Receiver<IT>,
    _phantom: PhantomData<E>
}


pub type SyncCommEnv<Spec> = SyncComm<EnvMessage<Spec>, AgentMessage<Spec>, CommError<Spec>>;
pub type SyncCommAgent<Spec> = SyncComm<AgentMessage<Spec>, EnvMessage<Spec>,  CommError<Spec>>;

impl<OT, IT, E: Error> SyncComm<OT, IT, E>
where SyncComm<OT, IT, E> :  CommEndpoint<OutwardType = OT, InwardType = IT, Error = E>{
    pub fn new(sender: SyncSender<OT>, receiver: Receiver<IT>) -> Self{
        Self{sender, receiver, _phantom: PhantomData::default()}
    }
    pub fn new_pair() -> (Self, SyncComm<IT, OT, E>) {
        let (tx_1, rx_1) = sync_channel(1);
        let (tx_2, rx_2) = sync_channel(1);

        (Self{sender: tx_1, receiver: rx_2, _phantom: PhantomData::default()},
        SyncComm{sender: tx_2, receiver: rx_1, _phantom: PhantomData::default()})
    }
    pub fn _decompose(self) -> (SyncSender<OT>, Receiver<IT>){
        (self.sender, self.receiver)
    }
}

impl<OT, IT, E> CommEndpoint for SyncComm<OT, IT, E>
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

    fn receive_non_blocking(&mut self) -> Result<IT, E> {
        self.receiver.try_recv().map_err(|e| e.into())
    }


}


pub enum DynComm<OT, IT, E: Error>{
    Std(SyncComm<OT, IT, E>),
    Dynamic(Box<dyn CommEndpoint<OutwardType = OT, InwardType = IT, Error = E>>)
}

impl <OT: Debug, IT: Debug, E: Error> CommEndpoint for DynComm<OT, IT, E>
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

    fn receive_non_blocking(&mut self) -> Result<Self::InwardType, Self::Error> {
        match self{
            DynComm::Std(c) => c.receive_non_blocking(),
            DynComm::Dynamic(c) => {c.as_mut().receive_non_blocking()}
        }
    }
}

