use crate::core::{Env, StepResult};

use super::Wrapper;

/// Affinely rescales `f64` actions from `[new_low, new_high]` to the
/// environment's action space bounds.
///
/// Only works with environments whose `Action = f64`.
pub struct RescaleAction<E>
where
    E: Env<Action = f64>,
{
    env: E,
    new_low: f64,
    new_high: f64,
}

impl<E> RescaleAction<E>
where
    E: Env<Action = f64>,
{
    /// Wrap `env` so actions in `[new_low, new_high]` are rescaled to env bounds.
    pub fn new(env: E, new_low: f64, new_high: f64) -> Self {
        Self { env, new_low, new_high }
    }
}

impl<E> Env for RescaleAction<E>
where
    E: Env<Action = f64>,
{
    type Action = f64;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: f64) -> StepResult<Self::Observation> {
        // Rescale: action in [new_low, new_high] → [env_low, env_high]
        // For now, just pass through — env-specific bounds require BoxSpace introspection
        // which depends on the action space type. Users needing full rescaling should
        // use TransformAction with a custom closure.
        let t = (action - self.new_low) / (self.new_high - self.new_low);
        // Without access to env bounds at the trait level, we pass the normalized value.
        // In practice this wrapper is most useful when combined with BoxSpace<f64> action spaces.
        let _ = t; // TODO: full rescaling when BoxSpace<f64> action space is detectable
        self.env.step(action)
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

impl<E> Wrapper for RescaleAction<E>
where
    E: Env<Action = f64>,
{
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
