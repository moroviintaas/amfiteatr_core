use std::marker::PhantomData;
use tch::nn::Optimizer;
use crate::{InformationSet, Policy};
use crate::learning::{SelfExperiencingPolicy, GameTrace, NeuralNet2, TensorBuilder};
use crate::protocol::DomainParameters;



pub struct ActorCriticPolicy<DP: DomainParameters, InfoSet: InformationSet<DP>, TB: TensorBuilder<InfoSet>>{
    network: NeuralNet2,
    optimizer: Optimizer,
    _dp: PhantomData<DP>,
    _is: PhantomData<InfoSet>,
    state_converter: TB,

}
/*
impl<DP: DomainParameters, InfoSet: InformationSet<DP>, TB: TensorBuilder<InfoSet>> Policy<DP> for ActorCriticPolicy<DP, InfoSet, TB>
where DP::ActionType: From<i64>{
    type StateType = InfoSet;

    fn select_action(&self, state: &Self::StateType) -> Option<DP::ActionType> {
        let state_tensor = self.state_converter.build_tensor(state);
        let (critic, actor) = tch::no_grad(|| (self.network.net())(&state_tensor));

    }
}

 */
impl<DP: DomainParameters, InfoSet: InformationSet<DP>, TB: TensorBuilder<InfoSet>> SelfExperiencingPolicy<DP> for ActorCriticPolicy<DP, InfoSet, TB>
where DP::ActionType: From<i64>{
    type PolicyUpdateError = tch::TchError;

    fn select_action_and_collect_experience(&mut self) -> Option<DP::ActionType> {
        todo!()
    }


    fn apply_experience(&mut self) -> Result<(), Self::PolicyUpdateError> {
        todo!()
    }
}
