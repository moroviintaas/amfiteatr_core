
/// Trait for constructing structs from another.
/// This is very similar to trait `From<T>`.
/// New trait is introduced to allow blanket implementation for Box<> and
pub trait Construct<S>{

    fn construct_from(base: S) -> Self;

    fn construct_similar_from(&mut self, base: S) -> Self where Self: Sized{
        Self::construct_from(base)
    }

}



pub trait Renew<S>{

    fn renew_from(&mut self, base: S);



}

impl<S, T: Construct<S>> Construct<S> for Box<T>{

    fn construct_from(base: S) -> Self {
        Box::new(T::construct_from(base))
    }


}