use std::fmt::Debug;

/// A space describes the set of valid values for actions or observations.
pub trait Space: Clone + Debug {
    /// The type of elements in this space.
    type Element;

    /// Check whether `value` belongs to this space.
    fn contains(&self, value: &Self::Element) -> bool;
}
