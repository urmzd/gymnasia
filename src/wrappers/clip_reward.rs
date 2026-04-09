use crate::core::{Env, StepResult};

use super::Wrapper;

/// Clips the reward to a `[min, max]` range.
pub struct ClipReward<E: Env> {
    env: E,
    min_reward: f64,
    max_reward: f64,
}

impl<E: Env> ClipReward<E> {
    /// Wrap `env` with reward clipping.
    pub fn new(env: E, min_reward: f64, max_reward: f64) -> Self {
        Self {
            env,
            min_reward,
            max_reward,
        }
    }
}

impl<E: Env> Env for ClipReward<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        result.reward = result.reward.clamp(self.min_reward, self.max_reward);
        result
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

impl<E: Env> Wrapper for ClipReward<E> {
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
