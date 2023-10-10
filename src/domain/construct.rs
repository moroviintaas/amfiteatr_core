pub trait Construct<S>{

    fn construct_from(base: S) -> Self;

    fn construct_similar_from(&mut self, base: S) -> Self where Self: Sized{
        Self::construct_from(base)
    }

}

impl<S, T: Construct<S>> Construct<S> for Box<T>{

    fn construct_from(base: S) -> Self {
        Box::new(T::construct_from(base))
    }


}