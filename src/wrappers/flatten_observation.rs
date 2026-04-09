use crate::core::{Env, Flatten, StepResult};
use crate::spaces::BoxSpace;

use super::Wrapper;

/// Flattens observations to `Vec<f64>` using the [`Flatten`] trait.
///
/// Changes the observation type from `E::Observation` to `Vec<f64>`.
pub struct FlattenObservation<E: Env>
where
    E::Observation: Flatten,
{
    env: E,
}

impl<E: Env> FlattenObservation<E>
where
    E::Observation: Flatten,
{
    /// Wrap `env` with observation flattening.
    pub fn new(env: E) -> Self {
        Self { env }
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
        // This is a simplification — the flattened observation space should be
        // a Box<f64> with appropriate low/high bounds derived from the inner space.
        // For now we return a reference which requires storing the flattened space.
        // TODO: Store flattened observation space on construction.
        unimplemented!("FlattenObservation::observation_space requires stored flattened space")
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
