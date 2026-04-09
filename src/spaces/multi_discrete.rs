use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use super::{sample_space::SampleSpace, space::Space};

/// Cartesian product of discrete spaces.
///
/// Each axis `i` has `nvec[i]` elements starting at `start[i]`.
/// Matches Gymnasium's `MultiDiscrete(nvec, start=...)`.
///
/// # Example
///
/// ```
/// use gymnasia::spaces::{MultiDiscrete, Space};
///
/// // Two axes: {0,1,2} x {0,1}
/// let space = MultiDiscrete::new(vec![3, 2]);
/// assert!(space.contains(&vec![2, 1]));
/// assert!(!space.contains(&vec![3, 0]));
/// ```
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct MultiDiscrete {
    /// Number of elements per axis.
    pub nvec: Vec<usize>,
    /// Starting value per axis.
    pub start: Vec<i64>,
}

impl MultiDiscrete {
    /// Create with zero-based starts.
    pub fn new(nvec: Vec<usize>) -> Self {
        let start = vec![0; nvec.len()];
        Self { nvec, start }
    }

    /// Create with custom start values per axis.
    pub fn with_start(nvec: Vec<usize>, start: Vec<i64>) -> Self {
        assert_eq!(nvec.len(), start.len());
        Self { nvec, start }
    }
}

impl Space for MultiDiscrete {
    type Element = Vec<i64>;

    fn contains(&self, value: &Vec<i64>) -> bool {
        if value.len() != self.nvec.len() {
            return false;
        }
        value
            .iter()
            .zip(self.nvec.iter().zip(self.start.iter()))
            .all(|(v, (n, s))| *v >= *s && *v < s + *n as i64)
    }
}

impl SampleSpace for MultiDiscrete {
    type Mask = Vec<Vec<bool>>;

    fn sample<R: Rng>(&self, rng: &mut R, mask: Option<&Self::Mask>) -> Vec<i64> {
        self.nvec
            .iter()
            .zip(self.start.iter())
            .enumerate()
            .map(|(i, (n, s))| {
                if let Some(mask) = mask {
                    let axis_mask = &mask[i];
                    let valid: Vec<i64> = (0..*n)
                        .filter(|&j| axis_mask.get(j).copied().unwrap_or(false))
                        .map(|j| s + j as i64)
                        .collect();
                    assert!(
                        !valid.is_empty(),
                        "mask must allow at least one value per axis"
                    );
                    let idx = Uniform::new(0, valid.len()).sample(rng);
                    valid[idx]
                } else {
                    let idx = Uniform::new(0, *n).sample(rng);
                    s + idx as i64
                }
            })
            .collect()
    }
}
