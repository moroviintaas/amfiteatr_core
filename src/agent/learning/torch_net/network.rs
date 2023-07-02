use std::marker::PhantomData;
use tch::Tensor;

pub struct NeuralNet<Apply: Fn(&Tensor) -> Tensor + Send>{
    _phantom: PhantomData<Apply>,

}