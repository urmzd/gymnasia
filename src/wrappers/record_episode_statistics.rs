use crate::core::{Env, StepResult};

use super::Wrapper;

/// Tracks cumulative reward and episode length.
///
/// Access statistics via typed methods:
/// - [`episode_return`](RecordEpisodeStatistics::episode_return)
/// - [`episode_length`](RecordEpisodeStatistics::episode_length)
/// - [`episode_count`](RecordEpisodeStatistics::episode_count)
pub struct RecordEpisodeStatistics<E: Env> {
    env: E,
    episode_return: f64,
    episode_length: usize,
    episode_count: usize,
    /// Returns from completed episodes (ring buffer).
    return_history: Vec<f64>,
    /// Lengths from completed episodes (ring buffer).
    length_history: Vec<usize>,
    buffer_length: usize,
}

impl<E: Env> RecordEpisodeStatistics<E> {
    /// Wrap `env` with episode statistics tracking.
    pub fn new(env: E) -> Self {
        Self::with_buffer(env, 100)
    }

    /// Wrap with a custom history buffer length.
    pub fn with_buffer(env: E, buffer_length: usize) -> Self {
        Self {
            env,
            episode_return: 0.0,
            episode_length: 0,
            episode_count: 0,
            return_history: Vec::with_capacity(buffer_length),
            length_history: Vec::with_capacity(buffer_length),
            buffer_length,
        }
    }

    /// Cumulative reward for the current episode.
    pub fn episode_return(&self) -> f64 {
        self.episode_return
    }

    /// Number of steps in the current episode.
    pub fn episode_length(&self) -> usize {
        self.episode_length
    }

    /// Total number of completed episodes.
    pub fn episode_count(&self) -> usize {
        self.episode_count
    }

    /// Returns from recent completed episodes.
    pub fn return_history(&self) -> &[f64] {
        &self.return_history
    }

    /// Lengths of recent completed episodes.
    pub fn length_history(&self) -> &[usize] {
        &self.length_history
    }

    fn record_episode(&mut self) {
        self.episode_count += 1;
        if self.return_history.len() >= self.buffer_length {
            self.return_history.remove(0);
            self.length_history.remove(0);
        }
        self.return_history.push(self.episode_return);
        self.length_history.push(self.episode_length);
    }
}

impl<E: Env> Env for RecordEpisodeStatistics<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let result = self.env.step(action);
        self.episode_return += result.reward;
        self.episode_length += 1;
        if result.terminated || result.truncated {
            self.record_episode();
        }
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.episode_return = 0.0;
        self.episode_length = 0;
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

impl<E: Env> Wrapper for RecordEpisodeStatistics<E> {
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
