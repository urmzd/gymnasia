use derive_new::new;
use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    prelude::Distribution,
    Rng,
};
use serde::Serialize;

use super::{sample_space::SampleSpace, Space};

/// Defines a subspace created between two points.
#[derive(Debug, Serialize, new, Clone)]
pub struct BoxR<T> {
    /// Defines the lower bound of the subspace where values less than what
    /// is inputted cannot exist.
    pub low: T,
    /// Defines the upper bound of the subspace where values greater than what is
    /// inputted cannot exist.
    pub high: T,
}

impl<T: PartialOrd + PartialEq> Space<T> for BoxR<T> {
    fn contains(&self, value: T) -> bool {
        value >= self.low && value <= self.high
    }
}

impl<T: SampleUniform + PartialOrd + PartialEq + Copy> SampleSpace<T> for BoxR<T> {
    type Mask = ();

    fn sample<R: Rng>(&self, rng: &mut R, _mask: Option<&Self::Mask>) -> T {
        Uniform::new_inclusive(self.low, self.high).sample(rng)
    }
}
