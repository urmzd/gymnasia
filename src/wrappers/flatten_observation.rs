use crate::core::{Env, Flatten, StepResult};
use crate::spaces::{Bounded, BoxSpace};

use super::Wrapper;

/// Flattens observations to `Vec<f64>` using the [`Flatten`] trait.
///
/// Changes the observation type from `E::Observation` to `Vec<f64>`.
/// The flattened observation space is computed once at construction.
pub struct FlattenObservation<E: Env>
where
    E::Observation: Flatten,
{
    env: E,
    observation_space: BoxSpace<f64>,
}

impl<E: Env> FlattenObservation<E>
where
    E::Observation: Flatten + Bounded,
    E::ObservationSpace: AsRef<BoxSpace<E::Observation>>,
{
    /// Wrap `env` with observation flattening.
    ///
    /// Computes the flattened observation space from the inner space's
    /// low/high bounds via [`Flatten::flatten`].
    pub fn new(env: E) -> Self {
        let inner_space: &BoxSpace<E::Observation> = env.observation_space().as_ref();
        let flat_low = inner_space.low.flatten();
        let flat_high = inner_space.high.flatten();
        let lo = flat_low
            .iter()
            .copied()
            .reduce(f64::min)
            .unwrap_or(f64::NEG_INFINITY);
        let hi = flat_high
            .iter()
            .copied()
            .reduce(f64::max)
            .unwrap_or(f64::INFINITY);
        let observation_space = BoxSpace::new(lo, hi);
        Self {
            env,
            observation_space,
        }
    }
}

impl<E: Env> Env for FlattenObservation<E>
where
    E::Observation: Flatten,
{
    type Action = E::Action;
    type Observation = Vec<f64>;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = BoxSpace<f64>;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Vec<f64>> {
        let result = self.env.step(action);
        StepResult {
            observation: result.observation.flatten(),
            reward: result.reward,
            terminated: result.terminated,
            truncated: result.truncated,
        }
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Vec<f64> {
        self.env.reset(seed, options).flatten()
    }

    fn action_space(&self) -> &Self::ActionSpace {
        self.env.action_space()
    }

    fn observation_space(&self) -> &BoxSpace<f64> {
        &self.observation_space
    }

    fn close(&mut self) {
        self.env.close();
    }
}

impl<E: Env> Wrapper for FlattenObservation<E>
where
    E::Observation: Flatten,
{
    type Inner = E;
    fn inner(&self) -> &E {
        &self.env
    }
    fn inner_mut(&mut self) -> &mut E {
        &mut self.env
    }
    fn into_inner(self) -> E {
        self.env
    }
}
