use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use super::{discrete_range::DiscreteRange, sample_space::SampleSpace, Space};

/// Defines a set of discrete integers starting at 0.
///
/// The value held by this structure defines the largest inclusive value that
/// exists within the derived set.  
///
/// TODO: Update to support negative values.
#[derive(Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Discrete(pub usize);

impl Discrete {
    /// Creates a [`DiscreteRange`] with `n` elements starting at `start`.
    ///
    /// This bridges to the Gymnasium v1.x `Discrete(n, start=...)` constructor.
    pub fn with_start(n: usize, start: isize) -> DiscreteRange {
        DiscreteRange::new(n, start)
    }
}

impl Space<usize> for Discrete {
    fn contains(&self, value: usize) -> bool {
        match *self {
            Discrete(upper_bound) => value < upper_bound,
        }
    }
}

impl SampleSpace<usize> for Discrete {
    type Mask = Vec<bool>;

    fn sample<R: Rng>(&self, rng: &mut R, mask: Option<&Self::Mask>) -> usize {
        let Discrete(n) = *self;
        if let Some(mask) = mask {
            let valid: Vec<usize> = (0..n)
                .filter(|&i| mask.get(i).copied().unwrap_or(false))
                .collect();
            assert!(!valid.is_empty(), "mask must allow at least one action");
            let idx = Uniform::new(0, valid.len()).sample(rng);
            valid[idx]
        } else {
            Uniform::new(0, n).sample(rng)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Discrete;
    use crate::spaces::Space;

    #[test]
    fn given_value_greater_or_eq_than_upper_bound_when_contains_called_then_returns_false() {
        let obj = Discrete(3);

        assert!(!obj.contains(3));
        assert!(!obj.contains(4));
    }

    #[test]
    fn given_value_less_than_upper_bound_when_contains_then_returns_true() {
        let obj = Discrete(3);

        assert!(obj.contains(1));
        assert!(obj.contains(2));
    }
}
