use ordered_float::OrderedFloat;

/// The standard float type used internally by environments.
///
/// Wraps `f64` to provide `Ord` + `Eq` (which raw `f64` lacks due to NaN).
pub type O64 = OrderedFloat<f64>;
