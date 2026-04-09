use super::step_result::StepResult;

/// The core environment interface.
///
/// This trait defines the minimal contract for reinforcement learning
/// environments. It has **no supertraits** — concrete types derive
/// `Clone`, `Debug`, `Serialize`, etc. as needed.
///
/// # Gymnasium compliance
///
/// Maps to Python Gymnasium's `gymnasium.Env` with Rust-idiomatic signatures:
///
/// | Gymnasium | gymnasia |
/// |-----------|----------|
/// | `step(action) → (obs, reward, term, trunc, info)` | `step(action) → StepResult<Obs>` |
/// | `reset(seed, options) → (obs, info)` | `reset(seed, options) → Obs` |
/// | `close()` | `close()` |
/// | `action_space` | `action_space()` |
/// | `observation_space` | `observation_space()` |
///
/// **Info** is intentionally absent from `StepResult`. Wrappers that track
/// metadata (e.g. episode statistics) expose it via typed methods on the
/// wrapper itself — see the `wrappers` module.
pub trait Env {
    /// The type of action accepted by [`step`](Env::step).
    type Action;

    /// The type of observation returned by [`step`](Env::step) and [`reset`](Env::reset).
    type Observation;

    /// The space describing valid actions.
    type ActionSpace;

    /// The space describing valid observations.
    type ObservationSpace;

    /// Environment-specific options passed to [`reset`](Env::reset).
    ///
    /// Use `()` if the environment has no configurable reset options.
    /// Use `Option<Box<Obs>>` (or similar) for environments that accept
    /// initial-state bounds.
    type ResetOptions: Default;

    /// Advance the environment by one step.
    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation>;

    /// Reset the environment to an initial state.
    ///
    /// If `seed` is `Some`, the environment's PRNG is re-seeded for
    /// reproducibility. `options` provides environment-specific reset
    /// configuration (pass `Default::default()` for defaults).
    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation;

    /// The action space descriptor.
    fn action_space(&self) -> &Self::ActionSpace;

    /// The observation space descriptor.
    fn observation_space(&self) -> &Self::ObservationSpace;

    /// Release resources held by the environment.
    ///
    /// The default implementation is a no-op.
    fn close(&mut self) {}
}
