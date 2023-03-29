use crate::state::agent::InformationSet;
use crate::state::State;

pub trait StatefulAgent{
    type State: InformationSet;

    fn update(&mut self, state_update: <Self::State as State>::UpdateType) -> Result<(), <Self::State as State>::Error>;
    fn state(&self) -> &Self::State;
}