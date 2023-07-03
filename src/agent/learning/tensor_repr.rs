use tch::Tensor;

pub trait TensorBuilder<T>{
    type Error: std::error::Error;
    fn build_tensor(&self, t: &T) -> Result<Tensor, Self::Error>;
}

pub trait TensorInterpreter<T>{
    type Error: std::error::Error;
    fn interpret_tensor(&self, tensor: &Tensor) -> Result<T, Self::Error>;
}
