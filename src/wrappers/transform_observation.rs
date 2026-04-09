use crate::core::{Env, StepResult};

use super::Wrapper;

/// Applies a user-provided function to transform observations.
///
/// Can change the observation type (e.g. from a struct to a processed form).
pub struct TransformObservation<E: Env, F, O> {
    env: E,
    func: F,
    _marker: std::marker::PhantomData<O>,
}

impl<E, F, O> TransformObservation<E, F, O>
where
    E: Env,
    F: Fn(E::Observation) -> O,
{
    /// Wrap `env` with an observation transformation.
    ///
    /// `func` converts the inner env's observation type into `O`.
    pub fn new(env: E, func: F) -> Self {
        Self {
            env,
            func,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E, F, O> Env for TransformObservation<E, F, O>
where
    E: Env,
    F: Fn(E::Observation) -> O,
{
    type Action = E::Action;
    type Observation = O;
    type ActionSpace = E::ActionSpace;
    // Observation space type doesn't change — the user is responsible for
    // interpreting the transformed observations correctly.
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<O> {
        let result = self.env.step(action);
        StepResult {
            observation: (self.func)(result.observation),
            reward: result.reward,
            terminated: result.terminated,
            truncated: result.truncated,
        }
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> O {
        let obs = self.env.reset(seed, options);
        (self.func)(obs)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

impl<E, F, O> Wrapper for TransformObservation<E, F, O>
where
    E: Env,
    F: Fn(E::Observation) -> O,
{
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
