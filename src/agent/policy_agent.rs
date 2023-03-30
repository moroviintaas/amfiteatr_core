use crate::action::Action;

pub trait ActingAgent {
    type Act: Action;

    fn take_action(&self) -> Option<Self::Act>;
}