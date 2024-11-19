//! Operations for creating, transforming, and modifying objects.
//
pub mod boolean;
pub mod transform;
///
/// Algorithm to build a polygon.
pub trait Polygon<T> {
    type Error;
    ///
    /// Creates a polygon.
    fn polygon(iter: impl IntoIterator<Item = T>, closed: bool) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
