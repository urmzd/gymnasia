use crate::core::{Env, StepResult};

use super::Wrapper;

/// Applies a user-provided function to the reward.
pub struct TransformReward<E: Env, F: Fn(f64) -> f64> {
    env: E,
    func: F,
}

impl<E: Env, F: Fn(f64) -> f64> TransformReward<E, F> {
    /// Wrap `env` with a reward transformation function.
    pub fn new(env: E, func: F) -> Self {
        Self { env, func }
    }
}

impl<E: Env, F: Fn(f64) -> f64> Env for TransformReward<E, F> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        result.reward = (self.func)(result.reward);
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

impl<E: Env, F: Fn(f64) -> f64> Wrapper for TransformReward<E, F> {
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
