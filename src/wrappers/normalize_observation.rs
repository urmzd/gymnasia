use crate::core::{Env, Flatten, StepResult};

use super::Wrapper;

/// Normalizes observations to approximately zero mean and unit variance
/// using running statistics.
///
/// Requires `E::Observation: Flatten` — normalization operates on the
/// flat representation and reconstructs the typed observation.
///
/// Statistics are initialized lazily from the first observation, so this
/// wrapper works regardless of what `flat_dim()` returns.
pub struct NormalizeObservation<E: Env>
where
    E::Observation: Flatten,
{
    env: E,
    epsilon: f64,
    running_mean: Vec<f64>,
    running_var: Vec<f64>,
    count: f64,
    initialized: bool,
}

impl<E: Env> NormalizeObservation<E>
where
    E::Observation: Flatten,
{
    /// Wrap `env` with observation normalization.
    pub fn new(env: E) -> Self {
        Self::with_epsilon(env, 1e-8)
    }

    /// Wrap with a custom epsilon for numerical stability.
    pub fn with_epsilon(env: E, epsilon: f64) -> Self {
        Self {
            env,
            epsilon,
            running_mean: Vec::new(),
            running_var: Vec::new(),
            count: 0.0,
            initialized: false,
        }
    }

    /// Current running mean.
    pub fn running_mean(&self) -> &[f64] {
        &self.running_mean
    }

    /// Current running variance.
    pub fn running_var(&self) -> &[f64] {
        &self.running_var
    }

    fn ensure_initialized(&mut self, dim: usize) {
        if !self.initialized {
            self.running_mean = vec![0.0; dim];
            self.running_var = vec![1.0; dim];
            self.initialized = true;
        }
    }

    fn normalize(&mut self, obs: E::Observation) -> E::Observation {
        let flat = obs.flatten();
        self.ensure_initialized(flat.len());

        // Welford online update
        self.count += 1.0;
        for (i, &val) in flat.iter().enumerate() {
            let delta = val - self.running_mean[i];
            self.running_mean[i] += delta / self.count;
            let delta2 = val - self.running_mean[i];
            self.running_var[i] =
                (self.running_var[i] + (delta * delta2 - self.running_var[i]) / self.count)
                    .max(0.0);
        }

        // Normalize using updated statistics
        let normalized: Vec<f64> = flat
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                (val - self.running_mean[i]) / (self.running_var[i].sqrt() + self.epsilon)
            })
            .collect();

        E::Observation::unflatten(&normalized)
    }
}

impl<E: Env> Env for NormalizeObservation<E>
where
    E::Observation: Flatten,
{
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let mut result = self.env.step(action);
        result.observation = self.normalize(result.observation);
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        let obs = self.env.reset(seed, options);
        self.normalize(obs)
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

impl<E: Env> Wrapper for NormalizeObservation<E>
where
    E::Observation: Flatten,
{
    type Inner = E;
    fn inner(&self) -> &E {
        &self.env
    }
    fn inner_mut(&mut self) -> &mut E {
        &mut self.env
    }
    fn into_inner(self) -> E {
        self.env
    }
}
