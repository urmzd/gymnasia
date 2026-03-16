use rand::Rng;

/// A trait for spaces that can generate random samples, optionally constrained by a mask.
pub trait SampleSpace<T> {
    /// The type used to constrain sampling (e.g., a boolean mask for discrete spaces).
    type Mask;

    /// Draws a random sample from the space.
    ///
    /// If `mask` is provided, sampling is restricted to the elements allowed by the mask.
    fn sample<R: Rng>(&self, rng: &mut R, mask: Option<&Self::Mask>) -> T;
}
