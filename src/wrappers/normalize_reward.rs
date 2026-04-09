use crate::core::{Env, StepResult};

use super::Wrapper;

/// Normalizes rewards using a running mean and variance estimate.
///
/// Uses the Welford online algorithm for numerically stable computation.
pub struct NormalizeReward<E: Env> {
    env: E,
    gamma: f64,
    epsilon: f64,
    running_mean: f64,
    running_var: f64,
    count: f64,
    returns: f64,
}

impl<E: Env> NormalizeReward<E> {
    /// Wrap `env` with reward normalization.
    pub fn new(env: E) -> Self {
        Self::with_params(env, 0.99, 1e-8)
    }

    /// Wrap with custom discount factor and epsilon.
    pub fn with_params(env: E, gamma: f64, epsilon: f64) -> Self {
        Self {
            env,
            gamma,
            epsilon,
            running_mean: 0.0,
            running_var: 1.0,
            count: 0.0,
            returns: 0.0,
        }
    }

    /// Current running mean.
    pub fn running_mean(&self) -> f64 {
        self.running_mean
    }

    /// Current running variance.
    pub fn running_var(&self) -> f64 {
        self.running_var
    }

    fn update_and_normalize(&mut self, reward: f64, terminated: bool) -> f64 {
        if terminated {
            self.returns = 0.0;
        }
        self.returns = self.returns * self.gamma + reward;

        // Welford online update
        self.count += 1.0;
        let delta = self.returns - self.running_mean;
        self.running_mean += delta / self.count;
        let delta2 = self.returns - self.running_mean;
        self.running_var += (delta * delta2 - self.running_var) / self.count;

        reward / (self.running_var.sqrt() + self.epsilon)
    }
}

impl<E: Env> Env for NormalizeReward<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        result.reward = self.update_and_normalize(result.reward, result.terminated);
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        self.returns = 0.0;
        self.env.reset(seed, options)
    }

    fn action_space(&self) -> &Self::ActionSpace { self.env.action_space() }
    fn observation_space(&self) -> &Self::ObservationSpace { self.env.observation_space() }
    fn close(&mut self) { self.env.close(); }
}

impl<E: Env> Wrapper for NormalizeReward<E> {
    type Inner = E;
    fn inner(&self) -> &E { &self.env }
    fn inner_mut(&mut self) -> &mut E { &mut self.env }
    fn into_inner(self) -> E { self.env }
}
