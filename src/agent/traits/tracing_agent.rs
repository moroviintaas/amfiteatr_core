use crate::agent::{Agent, AgentTrajectory};
use crate::agent::info_set::ScoringInformationSet;
use crate::domain::DomainParameters;


/// Agent that collects game trajectory, which contains recorded information sets
/// in the moment of making decisions and collected rewards on the way to the end game.
pub trait TracingAgent<DP: DomainParameters, S: ScoringInformationSet<DP>>: Agent<DP>{
    /// Resets recorded trajectory
    fn reset_trajectory(&mut self);
    /// Moves out recorded trajectory leaving new initialized in place
    fn take_trajectory(&mut self) -> AgentTrajectory<DP, S>;
    //fn set_new_state(&mut self);
    /// Returns reference to held trajectory.
    fn game_trajectory(&self) -> &AgentTrajectory<DP, S>;
    /// Adds new record to stored trajectory, information set before taking action, and
    /// rewards in which resulted performed action.
    fn commit_trace(&mut self);
    /// Add explicit part of subjective reward.
    /// One part of reward is calculated based from information set.
    /// This is meant for adding explicit modification to this value.
    /// > It could be used in some reinforcement learning scenarios when selected action is filtered
    /// before sent to environment. One may want to force agent to change action if it is illegal.
    /// Doing so before sending action to environment takes away experience gained by performing illegal action.
    /// If filter blocks action and asks for another before sending to environment agent can record on
    /// trace explicit penalty and thus store information that action is bad while avoiding causing error in game.
    fn explicit_add_subjective_reward(&mut self, explicit: S::RewardType);
    //fn mark_last_action_illegal(&mut self);

}