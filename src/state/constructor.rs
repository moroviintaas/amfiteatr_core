use std::process::Output;

pub trait Constructor{
    type Seed;
    type Output;

    fn use_ref(&self, seed: &Self::Seed) -> Output;
    fn use_owned(&self, seed: Self::Seed) -> Output;
}