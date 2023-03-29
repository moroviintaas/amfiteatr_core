use std::fmt::{Debug, Display};

pub trait Action: Debug + Send + Clone + Display{}