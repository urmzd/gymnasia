use rand::Rng;
use serde::Serialize;

use super::{sample_space::SampleSpace, space::Space};

/// An n-dimensional binary space `{0, 1}^n`.
///
/// Matches Gymnasium's `MultiBinary(n)`.
///
/// # Example
///
/// ```
/// use gymnasia::spaces::{MultiBinary, Space};
///
/// let space = MultiBinary::new(3);
/// assert!(space.contains(&vec![0, 1, 1]));
/// assert!(!space.contains(&vec![0, 2, 1]));
/// assert!(!space.contains(&vec![0, 1])); // wrong length
/// ```
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct MultiBinary {
    /// Number of binary dimensions.
    pub n: usize,
}

impl MultiBinary {
    /// Create a new binary space of dimension `n`.
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl Space for MultiBinary {
    type Element = Vec<u8>;

    fn contains(&self, value: &Vec<u8>) -> bool {
        value.len() == self.n && value.iter().all(|&v| v == 0 || v == 1)
    }
}

impl SampleSpace for MultiBinary {
    type Mask = ();

    fn sample<R: Rng>(&self, rng: &mut R, _mask: Option<&Self::Mask>) -> Vec<u8> {
        (0..self.n).map(|_| rng.gen_range(0..=1)).collect()
    }
}
