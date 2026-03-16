mod box_r;
mod discrete;
/// Discrete space with configurable start offset.
pub mod discrete_range;
/// Trait for sampling from spaces with optional masks.
pub mod sample_space;
mod space;

pub use box_r::BoxR;
pub use discrete::Discrete;
pub use discrete_range::DiscreteRange;
pub use sample_space::SampleSpace;
pub use space::Space;
