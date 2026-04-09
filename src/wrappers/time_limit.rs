use crate::core::{Env, StepResult};

use super::Wrapper;

/// Truncates episodes after a fixed number of steps.
///
/// After `max_episode_steps` calls to [`step`](Env::step), the returned
/// `StepResult` will have `truncated = true` (unless the episode already
/// terminated naturally).
pub struct TimeLimit<E: Env> {
    env: E,
    max_episode_steps: usize,
    current_step: usize,
}

impl<E: Env> TimeLimit<E> {
    /// Wrap `env` with a step limit.
    pub fn new(env: E, max_episode_steps: usize) -> Self {
        Self {
            env,
            max_episode_steps,
            current_step: 0,
        }
    }

    /// Number of steps remaining before truncation.
    pub fn steps_remaining(&self) -> usize {
        self.max_episode_steps.saturating_sub(self.current_step)
    }

    /// The configured maximum steps per episode.
    pub fn max_steps(&self) -> usize {
        self.max_episode_steps
    }

    /// The current step count within the episode.
    pub fn current_step(&self) -> usize {
        self.current_step
    }
}

impl<E: Env> Env for TimeLimit<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        self.current_step += 1;
        if self.current_step >= self.max_episode_steps && !result.terminated {
            result.truncated = true;
        }
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.current_step = 0;
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

impl<E: Env> Wrapper for TimeLimit<E> {
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
