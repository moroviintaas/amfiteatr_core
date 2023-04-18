use std::error::Error;
use crate::protocol::ProtocolSpecification;
use crate::state::StateUpdate;

pub trait State<Spec: ProtocolSpecification>{
    //type UpdateType: StateUpdate;
    //type Error: Error + Clone;

    fn update(&mut self, update: Spec::UpdateType) -> Result<(), Spec::GameErrorType>;
    fn is_finished(&self) ->bool;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct UpdateHistory<U: StateUpdate>{
    updates: Vec<U>
}

impl<U: StateUpdate> UpdateHistory<U>{

    pub fn new_reserved(size: usize) -> Self{
        let mut updates = Vec::new();
        updates.reserve(size);
        Self{updates}
    }

    pub fn updates(&self) -> &Vec<U>{
        &self.updates
    }

    pub fn store_update(&mut self, update: U){
        self.updates.push(update)
    }
}