//!
//! Operations for creating, transforming, and modifying objects.
//
pub mod boolean;
pub mod transform;
///
/// Algorithm to build a polygon from a set of objects.
pub trait Polygon<T> {
    type Error;
    ///
    /// Creates a polygon from the iterator over objects of type `T`.
    ///
    /// Set `closed` to _true_ to get the closed result polygon,
    /// i. e. its start and end are the same objects.
    fn polygon(iter: impl IntoIterator<Item = T>, closed: bool) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
