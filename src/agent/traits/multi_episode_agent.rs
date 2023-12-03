use crate::agent::{AutomaticAgent, AutomaticAgentRewarded, ReseedAgent};
use crate::domain::DomainParameters;
use crate::error::AmfiError;



pub trait MultiEpisodeAgent<DP: DomainParameters, Seed>: ReseedAgent<DP, Seed> {

    fn store_episode(&mut self);
    fn clear_episodes(&mut self);



}

pub trait MultiEpisodeAutoAgent<DP: DomainParameters, Seed>: MultiEpisodeAgent<DP, Seed> + AutomaticAgent<DP>{
    fn run_episode(&mut self, seed: Seed) -> Result<(), AmfiError<DP>> {
        self.reseed(seed);
        self.run()?;
        self.store_episode();
        Ok(())
    }
}
impl <DP: DomainParameters, Seed, T: MultiEpisodeAgent<DP, Seed> + AutomaticAgent<DP>> MultiEpisodeAutoAgent<DP, Seed> for T{

}

pub trait MultiEpisodeAutoAgentRewarded<DP: DomainParameters, Seed>: MultiEpisodeAgent<DP, Seed> + AutomaticAgentRewarded<DP>{

    fn run_episode_rewarded(&mut self, seed: Seed) -> Result<(), AmfiError<DP>> {
        self.reseed(seed);
        self.run_rewarded()?;
        self.store_episode();
        Ok(())
    }
}

impl <DP: DomainParameters, Seed, T: MultiEpisodeAgent<DP, Seed> + AutomaticAgentRewarded<DP>> MultiEpisodeAutoAgentRewarded<DP, Seed> for T{

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