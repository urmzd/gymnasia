use crate::core::{Env, StepResult};

use super::{AsBoxBounds, Wrapper};

/// Affinely rescales `f64` actions from `[new_low, new_high]` to the
/// environment's action space bounds.
///
/// Requires `E::ActionSpace: AsBoxBounds<f64>` (satisfied by `BoxSpace<f64>`).
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
    ///
    /// # Panics
    ///
    /// Panics if `new_low >= new_high`.
    pub fn new(env: E, new_low: f64, new_high: f64) -> Self {
        assert!(
            new_low < new_high,
            "RescaleAction requires new_low < new_high, got {new_low} >= {new_high}"
        );
        Self {
            env,
            new_low,
            new_high,
        }
    }
}

impl<E> Env for RescaleAction<E>
where
    E: Env<Action = f64>,
    E::ActionSpace: AsBoxBounds<f64>,
{
    type Action = f64;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: f64) -> StepResult<Self::Observation> {
        let (env_low, env_high) = self.env.action_space().bounds();
        let t = (action - self.new_low) / (self.new_high - self.new_low);
        let rescaled = env_low + t * (env_high - env_low);
        self.env.step(rescaled)
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace {
        self.env.action_space()
    }
    fn observation_space(&self) -> &Self::ObservationSpace {
        self.env.observation_space()
    }
    fn close(&mut self) {
        self.env.close();
    }
}

impl<E> Wrapper for RescaleAction<E>
where
    E: Env<Action = f64>,
    E::ActionSpace: AsBoxBounds<f64>,
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
