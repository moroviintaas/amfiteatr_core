
/*/// Trait for constructing structs from another.
/// This is very similar to trait `From<T>`.
/// New trait is introduced to allow blanket implementation for Box<> and
pub trait From<S>{

    fn from(base: S) -> Self;

    fn construct_similar_from(&mut self, base: S) -> Self where Self: Sized{
        Self::from(base)
    }

}

 */


/// Trait for objects that can be renewed using some data.
/// For example agents can be renewed with new state for new game episode without changing
/// things that do not need to be changed (like communication interface or trajectory archive).
pub trait Renew<S>{

    fn renew_from(&mut self, base: S);



}
/*
impl<S, T: From<S>> From<S> for Box<T>{

    fn from(base: S) -> Self {
        Box::new(T::from(base))
    }


}

 */