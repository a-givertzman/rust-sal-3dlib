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
///
/// Algorithm to build a rectangle.
pub trait Rectangle<Center, Normal> {
    ///
    /// Creates a rectangle from given parameters.
    fn rect(center: &Center, normal: &Normal, height: f64, width: f64) -> Self;
}
///
/// Defines projection on the object.
pub trait Project<T> {
    type Error;
    //
    //
    fn project(&self, point: &T) -> Result<T, Self::Error>;
}
