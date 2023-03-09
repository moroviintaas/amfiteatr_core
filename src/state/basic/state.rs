use std::error::Error;
use crate::state::StateUpdate;

pub trait State{
    type UpdateType: StateUpdate;
    type Error: Error + Clone;

    fn update(&mut self, update: Self::UpdateType) -> Result<(), Self::Error>;
    fn is_finished(&self) ->bool;
}
