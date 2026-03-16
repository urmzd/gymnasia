use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use super::{sample_space::SampleSpace, Space};

/// A discrete space of integers `{start, start+1, ..., start+n-1}`.
///
/// This generalises [`Discrete`](super::Discrete) to support non-zero start values,
/// matching Gymnasium's `Discrete(n, start=...)` constructor.
#[derive(Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct DiscreteRange {
    /// The number of elements in the space.
    pub n: usize,
    /// The starting value (inclusive).
    pub start: isize,
}

impl DiscreteRange {
    /// Creates a new `DiscreteRange` with `n` elements starting at `start`.
    pub fn new(n: usize, start: isize) -> Self {
        Self { n, start }
    }
}

impl Space<isize> for DiscreteRange {
    fn contains(&self, value: isize) -> bool {
        value >= self.start && value < self.start + self.n as isize
    }
}

impl SampleSpace<isize> for DiscreteRange {
    type Mask = Vec<bool>;

    fn sample<R: Rng>(&self, rng: &mut R, mask: Option<&Self::Mask>) -> isize {
        if let Some(mask) = mask {
            let valid: Vec<isize> = (0..self.n)
                .filter(|&i| mask.get(i).copied().unwrap_or(false))
                .map(|i| self.start + i as isize)
                .collect();
            assert!(!valid.is_empty(), "mask must allow at least one action");
            let idx = Uniform::new(0, valid.len()).sample(rng);
            valid[idx]
        } else {
            let idx = Uniform::new(0, self.n).sample(rng);
            self.start + idx as isize
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    use super::*;

    #[test]
    fn contains_checks_range() {
        let space = DiscreteRange::new(3, 5); // {5, 6, 7}
        assert!(space.contains(5));
        assert!(space.contains(7));
        assert!(!space.contains(4));
        assert!(!space.contains(8));
    }

    #[test]
    fn sample_without_mask_stays_in_range() {
        let space = DiscreteRange::new(4, -2); // {-2, -1, 0, 1}
        let mut rng = Pcg64::seed_from_u64(42);
        for _ in 0..100 {
            let v = space.sample(&mut rng, None);
            assert!(space.contains(v), "{v} not in range");
        }
    }

    #[test]
    fn sample_with_mask_respects_mask() {
        let space = DiscreteRange::new(4, 0); // {0, 1, 2, 3}
        let mask = vec![false, true, false, true]; // only 1 and 3
        let mut rng = Pcg64::seed_from_u64(42);
        for _ in 0..100 {
            let v = space.sample(&mut rng, Some(&mask));
            assert!(v == 1 || v == 3, "got {v}, expected 1 or 3");
        }
    }
}
