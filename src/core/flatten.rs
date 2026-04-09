/// Bidirectional conversion between a typed value and a flat `f64` array.
///
/// Implement this on observation types to enable ML-pipeline interop
/// (e.g. feeding observations into neural networks). This trait is **not**
/// required by [`Env`](super::Env) — it is opt-in.
///
/// # Example
///
/// ```
/// use gymnasia::core::Flatten;
///
/// #[derive(Clone, Debug)]
/// struct MyObs { x: f64, y: f64 }
///
/// impl Flatten for MyObs {
///     fn flat_dim() -> usize { 2 }
///     fn flatten(&self) -> Vec<f64> { vec![self.x, self.y] }
///     fn unflatten(flat: &[f64]) -> Self {
///         assert_eq!(flat.len(), 2);
///         MyObs { x: flat[0], y: flat[1] }
///     }
/// }
/// ```
pub trait Flatten: Sized {
    /// Number of elements in the flat representation.
    fn flat_dim() -> usize;

    /// Convert to a flat `f64` array.
    fn flatten(&self) -> Vec<f64>;

    /// Reconstruct from a flat `f64` slice.
    ///
    /// # Panics
    ///
    /// Implementations should panic if `flat.len() != Self::flat_dim()`.
    fn unflatten(flat: &[f64]) -> Self;
}

impl Flatten for f64 {
    fn flat_dim() -> usize {
        1
    }

    fn flatten(&self) -> Vec<f64> {
        vec![*self]
    }

    fn unflatten(flat: &[f64]) -> Self {
        assert_eq!(flat.len(), 1);
        flat[0]
    }
}

impl Flatten for Vec<f64> {
    /// # Panics
    ///
    /// Always panics — `Vec<f64>` has no fixed compile-time dimension.
    /// Use `.flatten().len()` on an instance instead.
    fn flat_dim() -> usize {
        panic!("Vec<f64> has no fixed dimension; use .flatten().len() on an instance instead")
    }

    fn flatten(&self) -> Vec<f64> {
        self.clone()
    }

    fn unflatten(flat: &[f64]) -> Self {
        flat.to_vec()
    }
}
