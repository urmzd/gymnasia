//! Composable environment wrappers.
//!
//! Each wrapper is a generic struct `Foo<E: Env>` that implements [`Env`](crate::core::Env)
//! by delegating to an inner environment and intercepting specific methods.
//! Stacking wrappers produces nested types like
//! `RecordEpisodeStatistics<TimeLimit<CartPoleEnv>>`.
//!
//! Wrappers that track metadata (e.g. episode statistics) expose it via
//! typed methods on the wrapper itself — there is no dynamic info dict.

use crate::core::Env;

/// Provides access to the wrapper chain.
///
/// Wrappers that implement this trait allow users to traverse the
/// wrapper stack and access inner environments.
pub trait Wrapper: Env {
    /// The type of the immediately wrapped environment.
    type Inner: Env;

    /// Access the immediately wrapped environment.
    fn inner(&self) -> &Self::Inner;

    /// Mutable access to the inner environment.
    fn inner_mut(&mut self) -> &mut Self::Inner;

    /// Unwrap and return the inner environment.
    fn into_inner(self) -> Self::Inner;
}

/// Implements [`Env`] for a wrapper by delegating all methods to `self.env`.
///
/// Use this for wrappers that only need to override one or two methods.
/// After invoking this macro, override the methods you need by writing
/// a manual `impl` block.
///
/// # Usage
///
/// ```ignore
/// delegate_env!(MyWrapper<E>, env);
/// ```
///
/// This generates `impl<E: Env> Env for MyWrapper<E>` with all methods
/// forwarding to `self.env`.
#[macro_export]
macro_rules! delegate_env {
    ($wrapper:ident < $e:ident >, $field:ident) => {
        impl<$e: $crate::core::Env> $crate::core::Env for $wrapper<$e> {
            type Action = $e::Action;
            type Observation = $e::Observation;
            type ActionSpace = $e::ActionSpace;
            type ObservationSpace = $e::ObservationSpace;
            type ResetOptions = $e::ResetOptions;

            fn step(
                &mut self,
                action: Self::Action,
            ) -> $crate::core::StepResult<Self::Observation> {
                self.$field.step(action)
            }

            fn reset(
                &mut self,
                seed: Option<u64>,
                options: Self::ResetOptions,
            ) -> Self::Observation {
                self.$field.reset(seed, options)
            }

            fn action_space(&self) -> &Self::ActionSpace {
                self.$field.action_space()
            }

            fn observation_space(&self) -> &Self::ObservationSpace {
                self.$field.observation_space()
            }

            fn close(&mut self) {
                self.$field.close();
            }
        }
    };
}

// --- Concrete wrappers ---

mod autoreset;
mod clip_action;
mod clip_reward;
mod flatten_observation;
mod normalize_observation;
mod normalize_reward;
mod order_enforcing;
mod record_episode_statistics;
mod rescale_action;
mod time_limit;
mod transform_action;
mod transform_observation;
mod transform_reward;

pub use autoreset::Autoreset;
pub use clip_action::ClipAction;
pub use clip_reward::ClipReward;
pub use flatten_observation::FlattenObservation;
pub use normalize_observation::NormalizeObservation;
pub use normalize_reward::NormalizeReward;
pub use order_enforcing::OrderEnforcing;
pub use record_episode_statistics::RecordEpisodeStatistics;
pub use rescale_action::RescaleAction;
pub use time_limit::TimeLimit;
pub use transform_action::TransformAction;
pub use transform_observation::TransformObservation;
pub use transform_reward::TransformReward;
