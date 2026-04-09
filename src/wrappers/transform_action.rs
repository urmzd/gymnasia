use crate::core::{Env, StepResult};

use super::Wrapper;

/// Applies a user-provided function to transform actions before passing
/// them to the inner environment.
pub struct TransformAction<E: Env, F, A> {
    env: E,
    func: F,
    _marker: std::marker::PhantomData<A>,
}

impl<E, F, A> TransformAction<E, F, A>
where
    E: Env,
    F: Fn(A) -> E::Action,
{
    /// Wrap `env` with an action transformation.
    ///
    /// `func` converts the wrapper's action type `A` into the inner env's action type.
    pub fn new(env: E, func: F) -> Self {
        Self {
            env,
            func,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E, F, A> Env for TransformAction<E, F, A>
where
    E: Env,
    F: Fn(A) -> E::Action,
{
    type Action = A;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: A) -> StepResult<Self::Observation> {
        let transformed = (self.func)(action);
        self.env.step(transformed)
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

impl<E, F, A> Wrapper for TransformAction<E, F, A>
where
    E: Env,
    F: Fn(A) -> E::Action,
{
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
