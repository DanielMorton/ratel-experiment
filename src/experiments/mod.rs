use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

pub use multiple_runs::multiple_runs;
pub use pair_bernoulli::pool_bernoulli;

use ratel_bandit::{
    Agent, BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper, OptimisticAgent,
};

mod multiple_runs;
mod pair_bernoulli;
mod sequential_bernoulli;

fn random_init(rand_start: &Uniform<f64>, len: usize) -> Vec<f64> {
    (1..=len)
        .map(|_| rand_start.sample(&mut thread_rng()))
        .collect()
}
