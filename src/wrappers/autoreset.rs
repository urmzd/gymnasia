use crate::core::{Env, StepResult};

use super::Wrapper;

/// Automatically resets the environment when an episode ends.
///
/// When `step()` returns `terminated` or `truncated`, the environment is
/// immediately reset and the observation from the new episode is returned.
/// The terminal observation is stored and accessible via
/// [`last_terminal_observation`](Autoreset::last_terminal_observation).
pub struct Autoreset<E: Env> {
    env: E,
    last_terminal_obs: Option<E::Observation>,
}

impl<E: Env> Autoreset<E> {
    /// Wrap `env` with automatic resets.
    pub fn new(env: E) -> Self {
        Self {
            env,
            last_terminal_obs: None,
        }
    }

    /// The observation from the last terminal state, if any.
    pub fn last_terminal_observation(&self) -> Option<&E::Observation> {
        self.last_terminal_obs.as_ref()
    }
}

impl<E: Env> Env for Autoreset<E>
where
    E::Observation: Clone,
{
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        if result.terminated || result.truncated {
            self.last_terminal_obs = Some(result.observation.clone());
            let new_obs = self.env.reset(None, Default::default());
            result.observation = new_obs;
        }
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.last_terminal_obs = None;
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

impl<E: Env> Wrapper for Autoreset<E>
where
    E::Observation: Clone,
{
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
