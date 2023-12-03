use std::sync::{Arc, Mutex};
use crate::agent::{AutomaticAgentRewarded, ReseedAgent, StatefulAgent};
use crate::domain::DomainParameters;
use crate::error::AmfiError;



pub trait MultiEpisodeAgent<DP: DomainParameters, Seed>: ReseedAgent<DP, Seed> {

    fn store_episodes(&mut self);
    fn clear_episodes(&mut self);

    fn run_episode(&mut self, seed: Seed) -> Result<(), AmfiError<DP>>
    where Self: AutomaticAgentRewarded<DP>{
        self.reseed(seed);
        self.run()?;
        self.store_episodes();
        Ok(())
    }
    fn run_episode_rewarded(&mut self, seed: Seed) -> Result<(), AmfiError<DP>>
    where Self: AutomaticAgentRewarded<DP>{
        self.reseed(seed);
        self.run_rewarded()?;
        self.store_episodes();
        Ok(())
    }

}


/*

impl<DP: DomainParameters,  Seed,  T: MultiEpisodeAgent<DP, Seed>> MultiEpisodeAgent<DP, Seed> for Arc<Mutex<T>>{
    fn store_episodes(&mut self) {
        //let mut guard = self.get_mut().unwrap();
        let mut g = self.lock().unwrap();

        g.store_episodes()
    }

    fn clear_episodes(&mut self) {
        let mut guard = self.lock().unwrap();
        guard.clear_episodes()
    }
}


 */