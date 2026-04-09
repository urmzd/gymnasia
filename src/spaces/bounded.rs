use std::fmt::Debug;

use rand::Rng;
use serde::Serialize;

use super::{sample_space::SampleSpace, space::Space};

/// Types that can serve as element-wise bounds in a [`BoxSpace`].
///
/// Implement this on observation structs, scalar types, or array types
/// to use them with `BoxSpace<B>`. We ship implementations for `f64`
/// and [`Tensor`].
///
/// # Example
///
/// ```
/// use gymnasia::spaces::Bounded;
/// use rand::Rng;
///
/// #[derive(Clone, Debug)]
/// struct MyObs { x: f64, y: f64 }
///
/// impl Bounded for MyObs {
///     fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
///         value.x >= low.x && value.x <= high.x
///             && value.y >= low.y && value.y <= high.y
///     }
///     fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
///         MyObs {
///             x: rng.gen_range(low.x..=high.x),
///             y: rng.gen_range(low.y..=high.y),
///         }
///     }
/// }
/// ```
pub trait Bounded: Sized + Clone + Debug {
    /// Check whether `value` lies within `[low, high]` element-wise.
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool;

    /// Sample uniformly within `[low, high]` element-wise.
    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self;

    /// Clamp `value` element-wise to `[low, high]`.
    ///
    /// The default implementation returns `low` if out of bounds — override
    /// this for proper element-wise clamping on composite types.
    fn clamp(value: Self, low: &Self, high: &Self) -> Self {
        if Self::in_bounds(&value, low, high) {
            value
        } else {
            low.clone()
        }
    }
}

/// A bounded continuous space defined by low/high bounds.
///
/// Generic over the bounds representation `B`. Use typed observation
/// structs for classical control, [`Tensor`] for high-dimensional spaces,
/// or `f64` for scalar spaces.
///
/// # Examples
///
/// ```
/// use gymnasia::spaces::{BoxSpace, Space};
///
/// let space = BoxSpace::new(-1.0_f64, 1.0);
/// assert!(space.contains(&0.5));
/// assert!(!space.contains(&2.0));
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(bound = "B: Serialize")]
pub struct BoxSpace<B: Bounded> {
    /// The lower bound (inclusive).
    pub low: B,
    /// The upper bound (inclusive).
    pub high: B,
}

impl<B: Bounded> BoxSpace<B> {
    /// Create a new box space with the given bounds.
    pub fn new(low: B, high: B) -> Self {
        Self { low, high }
    }
}

impl<B: Bounded> AsRef<BoxSpace<B>> for BoxSpace<B> {
    fn as_ref(&self) -> &BoxSpace<B> {
        self
    }
}

impl<B: Bounded> Space for BoxSpace<B> {
    type Element = B;

    fn contains(&self, value: &B) -> bool {
        B::in_bounds(value, &self.low, &self.high)
    }
}

impl<B: Bounded> SampleSpace for BoxSpace<B> {
    type Mask = ();

    fn sample<R: Rng>(&self, rng: &mut R, _mask: Option<&Self::Mask>) -> B {
        B::sample_uniform(rng, &self.low, &self.high)
    }
}

// --- Shipped Bounded implementations ---

impl Bounded for f64 {
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
        value >= low && value <= high
    }

    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
        rng.gen_range(*low..=*high)
    }

    fn clamp(value: Self, low: &Self, high: &Self) -> Self {
        value.clamp(*low, *high)
    }
}

impl Bounded for f32 {
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
        value >= low && value <= high
    }

    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
        rng.gen_range(*low..=*high)
    }

    fn clamp(value: Self, low: &Self, high: &Self) -> Self {
        value.clamp(*low, *high)
    }
}

/// A flat array with shape metadata, for high-dimensional spaces.
///
/// Use this as the bound type in `BoxSpace<Tensor>` for image-based
/// or large state-vector observations.
#[derive(Clone, Debug, Serialize)]
pub struct Tensor {
    /// The flat data.
    pub data: Vec<f64>,
    /// The logical shape (e.g. `[210, 160, 3]` for an RGB image).
    pub shape: Vec<usize>,
}

impl Tensor {
    /// Create a new tensor with the given data and shape.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` does not equal the product of `shape`.
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Self {
        let expected: usize = shape.iter().product();
        assert_eq!(
            data.len(),
            expected,
            "data length {} does not match shape {:?} (expected {})",
            data.len(),
            shape,
            expected
        );
        Self { data, shape }
    }

    /// Create a tensor filled with a constant value.
    pub fn filled(value: f64, shape: Vec<usize>) -> Self {
        let n: usize = shape.iter().product();
        Self {
            data: vec![value; n],
            shape,
        }
    }
}

impl Bounded for Tensor {
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
        assert_eq!(value.shape, low.shape, "Tensor shape mismatch");
        assert_eq!(value.shape, high.shape, "Tensor shape mismatch");
        value
            .data
            .iter()
            .zip(low.data.iter().zip(high.data.iter()))
            .all(|(v, (lo, hi))| v >= lo && v <= hi)
    }

    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
        assert_eq!(low.shape, high.shape, "Tensor shape mismatch");
        let data: Vec<f64> = low
            .data
            .iter()
            .zip(high.data.iter())
            .map(|(lo, hi)| rng.gen_range(*lo..=*hi))
            .collect();
        Tensor {
            data,
            shape: low.shape.clone(),
        }
    }

    fn clamp(value: Self, low: &Self, high: &Self) -> Self {
        assert_eq!(value.shape, low.shape, "Tensor shape mismatch");
        let data: Vec<f64> = value
            .data
            .iter()
            .zip(low.data.iter().zip(high.data.iter()))
            .map(|(v, (lo, hi))| v.clamp(*lo, *hi))
            .collect();
        Tensor {
            data,
            shape: value.shape,
        }
    }
}
