use crate::core::{Env, StepResult};

use super::Wrapper;

/// Enforces that [`reset`](Env::reset) is called before [`step`](Env::step).
///
/// Panics if `step()` is called before the first `reset()`.
pub struct OrderEnforcing<E: Env> {
    env: E,
    has_reset: bool,
}

impl<E: Env> OrderEnforcing<E> {
    /// Wrap `env` with order enforcement.
    pub fn new(env: E) -> Self {
        Self {
            env,
            has_reset: false,
        }
    }
}

impl<E: Env> Env for OrderEnforcing<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        assert!(self.has_reset, "must call reset() before step()");
        self.env.step(action)
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.has_reset = true;
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

impl<E: Env> Wrapper for OrderEnforcing<E> {
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
