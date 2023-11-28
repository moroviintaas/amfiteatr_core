use crate::agent::{AutomaticAgent, AutomaticAgentRewarded};
use crate::domain::DomainParameters;
use crate::error::AmfiError;



pub trait MultiEpisodeAgent<DP: DomainParameters>{

    fn store_episodes(&mut self);
    fn clear_episodes(&mut self);

    fn run_episode(&mut self) -> Result<(), AmfiError<DP>>
    where Self: AutomaticAgentRewarded<DP>{
        self.run()?;
        self.store_episodes();
        Ok(())
    }
    fn run_episode_rewarded(&mut self) -> Result<(), AmfiError<DP>>
    where Self: AutomaticAgentRewarded<DP>{
        self.run_rewarded()?;
        self.store_episodes();
        Ok(())
    }

}
