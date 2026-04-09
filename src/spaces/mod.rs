/// Generic bounded continuous space.
mod bounded;
/// Discrete integer space.
mod discrete;
/// N-dimensional binary space.
mod multi_binary;
/// Cartesian product of discrete spaces.
mod multi_discrete;
/// Trait for sampling from spaces with optional masks.
mod sample_space;
/// Base space trait.
mod space;

pub use bounded::{Bounded, BoxSpace, Tensor};
pub use discrete::Discrete;
pub use multi_binary::MultiBinary;
pub use multi_discrete::MultiDiscrete;
pub use sample_space::SampleSpace;
pub use space::Space;
