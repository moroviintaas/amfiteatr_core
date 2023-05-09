


use crate::protocol::ProtocolSpecification;
use crate::Reward;
use crate::state::State;

pub trait InformationSet<Spec: ProtocolSpecification>: State<Spec>{
    //type ActionType: Action + Debug + Display;
    type ActionIteratorType: IntoIterator<Item = Spec::ActionType>;
    //type Id: AgentIdentifier;
    type RewardType: Reward;

    fn available_actions(&self) -> Self::ActionIteratorType;
    fn id(&self) -> &Spec::AgentId;
    fn is_action_valid(&self, action: &Spec::ActionType) -> bool;
    fn current_score(&self) -> Self::RewardType;
    fn final_score(&self) -> Option<Self::RewardType>{
        if self.is_finished(){
            Some(self.current_score())
        } else {
            None
        }
    }

}