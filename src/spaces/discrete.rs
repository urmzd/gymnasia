use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use super::{sample_space::SampleSpace, space::Space};

/// A discrete space of integers `{start, start+1, ..., start+n-1}`.
///
/// Matches Gymnasium's `Discrete(n, start=0)`.
///
/// # Examples
///
/// ```
/// use gymnasia::spaces::{Discrete, Space};
///
/// let space = Discrete::new(3); // {0, 1, 2}
/// assert!(space.contains(&0));
/// assert!(space.contains(&2));
/// assert!(!space.contains(&3));
///
/// let space = Discrete::with_start(3, 5); // {5, 6, 7}
/// assert!(space.contains(&5));
/// assert!(!space.contains(&4));
/// ```
#[derive(Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Discrete {
    /// Number of elements in the space.
    pub n: usize,
    /// Starting value (inclusive).
    pub start: i64,
}

impl Discrete {
    /// Create a space `{0, 1, ..., n-1}`.
    pub fn new(n: usize) -> Self {
        Self { n, start: 0 }
    }

    /// Create a space `{start, start+1, ..., start+n-1}`.
    pub fn with_start(n: usize, start: i64) -> Self {
        Self { n, start }
    }
}

impl Space for Discrete {
    type Element = i64;

    fn contains(&self, value: &i64) -> bool {
        *value >= self.start && *value < self.start + self.n as i64
    }
}

impl SampleSpace for Discrete {
    type Mask = Vec<bool>;

    fn sample<R: Rng>(&self, rng: &mut R, mask: Option<&Self::Mask>) -> i64 {
        if let Some(mask) = mask {
            let valid: Vec<i64> = (0..self.n)
                .filter(|&i| mask.get(i).copied().unwrap_or(false))
                .map(|i| self.start + i as i64)
                .collect();
            assert!(!valid.is_empty(), "mask must allow at least one action");
            let idx = Uniform::new(0, valid.len()).sample(rng);
            valid[idx]
        } else {
            let idx = Uniform::new(0, self.n).sample(rng);
            self.start + idx as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    use super::*;

    #[test]
    fn contains_zero_start() {
        let space = Discrete::new(3);
        assert!(space.contains(&0));
        assert!(space.contains(&2));
        assert!(!space.contains(&3));
        assert!(!space.contains(&-1));
    }

    #[test]
    fn contains_with_start() {
        let space = Discrete::with_start(3, 5); // {5, 6, 7}
        assert!(space.contains(&5));
        assert!(space.contains(&7));
        assert!(!space.contains(&4));
        assert!(!space.contains(&8));
    }

    #[test]
    fn contains_negative_start() {
        let space = Discrete::with_start(4, -2); // {-2, -1, 0, 1}
        assert!(space.contains(&-2));
        assert!(space.contains(&1));
        assert!(!space.contains(&-3));
        assert!(!space.contains(&2));
    }

    #[test]
    fn sample_without_mask_stays_in_range() {
        let space = Discrete::with_start(4, -2);
        let mut rng = Pcg64::seed_from_u64(42);
        for _ in 0..100 {
            let v = space.sample(&mut rng, None);
            assert!(space.contains(&v), "{v} not in range");
        }
    }

    #[test]
    fn sample_with_mask_respects_mask() {
        let space = Discrete::with_start(4, 0); // {0, 1, 2, 3}
        let mask = vec![false, true, false, true]; // only 1 and 3
        let mut rng = Pcg64::seed_from_u64(42);
        for _ in 0..100 {
            let v = space.sample(&mut rng, Some(&mask));
            assert!(v == 1 || v == 3, "got {v}, expected 1 or 3");
        }
    }
}
