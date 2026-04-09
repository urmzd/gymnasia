use crate::{
    core::{Env, StepResult},
    spaces::{Bounded, BoxSpace},
};

use super::Wrapper;

/// Clips continuous actions to the action space bounds.
///
/// Requires the environment's action type to implement [`Bounded`]
/// and the action space to be `BoxSpace<A>`.
pub struct ClipAction<E: Env> {
    env: E,
}

impl<E: Env> ClipAction<E>
where
    E::Action: Bounded,
{
    /// Wrap `env` with action clipping.
    pub fn new(env: E) -> Self {
        Self { env }
    }
}

impl<E> Env for ClipAction<E>
where
    E: Env,
    E::Action: Bounded,
    E::ActionSpace: AsBoxBounds<E::Action>,
{
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let space = self.env.action_space();
        let (low, high) = space.bounds();
        let clamped = if !E::Action::in_bounds(&action, low, high) {
            low.clone()
        } else {
            action
        };
        self.env.step(clamped)
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

/// Helper trait for extracting bounds from an action space.
pub trait AsBoxBounds<B: Bounded> {
    /// Get the low and high bounds.
    fn bounds(&self) -> (&B, &B);
}

impl<B: Bounded> AsBoxBounds<B> for BoxSpace<B> {
    fn bounds(&self) -> (&B, &B) {
        (&self.low, &self.high)
    }
}

impl<E> Wrapper for ClipAction<E>
where
    E: Env,
    E::Action: Bounded,
    E::ActionSpace: AsBoxBounds<E::Action>,
{
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
