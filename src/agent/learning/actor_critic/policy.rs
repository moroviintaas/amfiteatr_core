use std::fmt::Debug;
use std::marker::PhantomData;
use tch::Kind::Float;
use tch::nn::Optimizer;
use crate::{InformationSet, Policy};
use crate::learning::{SelfExperiencingPolicy, NeuralNet2, TensorBuilder, TensorInterpreter};
use crate::protocol::DomainParameters;



pub struct ActorCriticPolicy<
    DP: DomainParameters,
    InfoSet: InformationSet<DP> + Debug,
    StateConverter: TensorBuilder<InfoSet>,
    ActInterpreter: TensorInterpreter<Option<DP::ActionType>>
> {
    network: NeuralNet2,
    #[allow(dead_code)]
    optimizer: Optimizer,
    _dp: PhantomData<DP>,
    _is: PhantomData<InfoSet>,
    state_converter: StateConverter,
    action_interpreter: ActInterpreter

}

impl<
    DP: DomainParameters,
    InfoSet: InformationSet<DP> + Debug,
    StateConverter: TensorBuilder<InfoSet>,
    ActInterpreter: TensorInterpreter<Option<DP::ActionType>>
> ActorCriticPolicy<DP, InfoSet, StateConverter, ActInterpreter>{
    pub fn new(network: NeuralNet2,
               optimizer: Optimizer,
               state_converter: StateConverter,
               action_interpreter: ActInterpreter) -> Self{
        Self{network, optimizer, state_converter, action_interpreter, _dp: Default::default(), _is: Default::default()}
    }
}

impl<DP: DomainParameters,
    InfoSet: InformationSet<DP> + Debug,
    TB: TensorBuilder<InfoSet>,
    ActInterpreter: TensorInterpreter<Option<DP::ActionType>>
> Policy<DP> for ActorCriticPolicy<DP, InfoSet, TB, ActInterpreter>{
    type StateType = InfoSet;

    fn select_action(&self, state: &Self::StateType) -> Option<DP::ActionType> {
        let state_tensor = self.state_converter.build_tensor(state)
            .expect(&format!("Failed converting state to Tensor: {:?}", state));
        let (_critic, actor) = tch::no_grad(|| (self.network.net())(&state_tensor));

        //somewhen it may be changed with temperature
        let probs = actor.softmax(-1, Float);
        let atensor = probs.multinomial(1, true);
        let action_opt = self.action_interpreter.interpret_tensor(&atensor)
            .expect("Failed converting tensor to action");
        action_opt

    }
}


impl<
    DP: DomainParameters,
    InfoSet: InformationSet<DP> + Debug,
    TB: TensorBuilder<InfoSet>,
    ActInterpreter: TensorInterpreter<Option<DP::ActionType>>> SelfExperiencingPolicy<DP> for ActorCriticPolicy<DP, InfoSet, TB, ActInterpreter>
where DP::ActionType: From<i64>{
    type PolicyUpdateError = tch::TchError;

    fn select_action_and_collect_experience(&mut self) -> Option<DP::ActionType> {
        todo!()
    }


    fn apply_experience(&mut self) -> Result<(), Self::PolicyUpdateError> {
        todo!()
    }
}