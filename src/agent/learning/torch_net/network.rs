use std::marker::PhantomData;
use tch::{Device, TchError, Tensor};
use tch::nn::{Optimizer, OptimizerConfig, Path, Sequential, VarStore};
use crate::learning::SequentialBuilder;

pub struct NeuralNet{
    net: Box<dyn Fn(&Tensor) -> Tensor + Send>,
    var_store: VarStore,
    device: Device,


}

/// # Example:
/// ```
/// use sztorm::learning::{NeuralNet, SequentialBuilder};
/// use tch::{Device, nn};
/// use tch::nn::{Adam, VarStore};
/// let device = Device::cuda_if_available();
/// let var_store = VarStore::new(device);
/// let builder = SequentialBuilder::new(|path|{
///     nn::seq()
///         .add(nn::linear(path/"input", 10, 128, Default::default()))
///         .add(nn::linear(path/"hidden_1", 128, 2, Default::default()))
/// });
///
/// let neural_net = NeuralNet::new(&builder, var_store);
/// let optimiser = neural_net.build_optimiser(Adam::default(), 0.01);
/// ```
impl NeuralNet{

    pub fn new<F: Fn(&Path) -> Sequential>(sequential_builder: &SequentialBuilder<F>, var_store: VarStore) -> Self{
        let sequential = sequential_builder.build(&var_store.root());
        let device = var_store.root().device();
        Self{
            var_store,
            device,
            net: Box::new(move |x| {x.to_device(device).apply(&sequential)})
        }
    }
    pub fn build_optimiser<OptC: OptimizerConfig>
        (&self, optimiser_config: OptC, learning_rate: f64) -> Result<Optimizer, TchError>{

        optimiser_config.build(&self.var_store, learning_rate)
    }
}