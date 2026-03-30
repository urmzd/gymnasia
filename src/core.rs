use std::fmt::Debug;

use ordered_float::OrderedFloat;
use rand_pcg::Pcg64;
use serde::Serialize;

use crate::{
    spaces::BoxR,
    utils::custom::{draw::DrawList, traits::Sample, types::O64},
};

/// Defines a common set of operations available to different environments.
pub trait Env: Clone + Debug + Serialize
where
    Self::Observation: Sample + Into<Vec<f64>>,
{
    /// The type of action supported.
    type Action;

    /// The type of the observation produced after an action has been applied.
    type Observation;

    /// The type of the metadata object produced by acting on the environment.
    type Info;

    /// The type of the object produced when an environment is reset.
    type ResetInfo;

    /// The type of values that can be observed in the action space.
    type ActionSpace;

    /// The type of observations produced.
    type ObservationSpace;

    /// Acts on an environment using the given action, producing a reward.
    fn step(&mut self, action: Self::Action) -> ActionReward<Self::Observation, Self::Info>;

    /// Resets the environment to a initial random state.
    fn reset(
        &mut self,
        seed: Option<u64>,
        return_info: bool,
        options: Option<BoxR<Self::Observation>>,
    ) -> (Self::Observation, Option<Self::ResetInfo>);

    /// Provides the object describing the actions that can be observed.
    fn action_space(&self) -> &Self::ActionSpace;

    /// Provides the object describing the states that can be observed in this environment.
    fn observation_space(&self) -> &Self::ObservationSpace;

    /// Provides the random number generator responsible for seeding states.
    fn rand_random(&self) -> &Pcg64;

    /// Resets the environment and always returns info alongside the observation.
    fn reset_with_info(
        &mut self,
        seed: Option<u64>,
        options: Option<BoxR<Self::Observation>>,
    ) -> (Self::Observation, Self::ResetInfo)
    where
        Self::ResetInfo: Default,
    {
        let (obs, info) = self.reset(seed, true, options);
        (obs, info.unwrap_or_default())
    }
}

/// Environments that can produce a visual representation of their state.
///
/// [`DrawList`] is pure data with no rendering-backend dependency, so this
/// trait is always available regardless of feature flags.
pub trait Renderable {
    /// Produce draw commands representing the current visual state.
    /// The returned [`DrawList`] includes logical canvas width/height.
    fn draw_list(&self) -> DrawList;

    /// Target frames per second for human-mode display.
    fn render_fps(&self) -> u32 {
        30
    }
}

/// Encapsulates and describes the state update experienced by an environment after acting on an
/// action.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActionReward<T, E> {
    /// The current observable state.
    pub observation: T,
    /// The value of the reward produced.
    pub reward: O64,
    /// Indicates whether the episode has terminated (reached a terminal state) or not.
    pub terminated: bool,
    /// Indicates whether the episode has terminated early or not.
    pub truncated: bool,
    /// Additional info implementations may provide for purposes beyond classical RL.
    pub info: Option<E>,
}

/// Defines the bounds for the reward value that can be observed.
#[deprecated(
    since = "1.1.0",
    note = "Gymnasium v1.x no longer uses reward_range. Will be removed in a future version."
)]
#[derive(Clone, Debug, Serialize, PartialEq, Ord, PartialOrd, Eq)]
pub struct RewardRange {
    /// The smallest possible reward that can be observed.
    lower_bound: O64,
    /// The largest possible reward that can be observed.
    upper_bound: O64,
}

/// Implement a default reward range.
#[allow(deprecated)]
impl Default for RewardRange {
    fn default() -> Self {
        RewardRange {
            lower_bound: OrderedFloat(f64::NEG_INFINITY),
            upper_bound: OrderedFloat(f64::INFINITY),
        }
    }
}
