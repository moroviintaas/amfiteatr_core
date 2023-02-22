use crate::action::Action;

pub trait PolicyAgent{
    type Act: Action;

    fn select_action(&self) -> Option<Self::Act>;
}