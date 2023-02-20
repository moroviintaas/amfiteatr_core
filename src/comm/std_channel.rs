use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError, TryRecvError};
use crate::comm::endpoint::CommEndpoint;

#[derive(Debug)]
/// # Example:
/// ```
/// use std::thread::spawn;
/// use tur::comm::{CommEndpoint, CommError, SyncComm};
/// let (mut com1, mut com2) = SyncComm::<String, String, CommError>::new_pair();
/// let h1 = spawn(move || {
///     com1.send(format!("Hello")).unwrap();
/// });
/// let r = com2.recv().unwrap();
/// assert_eq!(r, format!("Hello"));
/// ```
pub struct SyncComm<OT, IT, E: Error>{
    sender: Sender<OT>,
    receiver: Receiver<IT>,
    _phantom: PhantomData<E>
}

impl<OT, IT, E: Error> SyncComm<OT, IT, E>
where SyncComm<OT, IT, E> :  CommEndpoint<OutwardType = OT, InwardType = IT, Error = E>{
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

impl<OT, IT, E> CommEndpoint for SyncComm<OT, IT, E>
where E: Debug + Error + From<RecvError> + From<SendError<OT>> + From<TryRecvError> + From<SendError<IT>>{
    type OutwardType = OT;

    type InwardType = IT;

    type Error = E;

    fn send(&mut self, message: OT) -> Result<(), E> {
        self.sender.send(message).map_err(|e| e.into())
    }

    fn recv(&mut self) -> Result<IT, E> {
        self.receiver.recv().map_err(|e| e.into())
    }

    fn try_recv(&mut self) -> Result<IT, E> {
        self.receiver.try_recv().map_err(|e| e.into())
    }


}

