//!
//! Rotate and move objects.
//
/// Defines the rotation of the object.
pub trait Rotate<O, A> {
    ///
    /// Consumes `self` and returns a new rotated instance, where
    /// - `origin` - rotation pivot,
    /// - `axis` - axis for rotation,
    /// - `rad` - angle in radians.
    fn rotate(self, origin: O, axis: A, rad: f64) -> Self;
}
///
/// Moving in space.
pub trait Translate<T> {
    ///
    /// Consumes `self` and returns a new translated instance moved to `dir`.
    fn translate(self, dir: T) -> Self;
}
