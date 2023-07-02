use tch::Tensor;

pub trait TensorBuilder<T>{
    fn build_tensor(&self, t: &T) -> Tensor;
}

