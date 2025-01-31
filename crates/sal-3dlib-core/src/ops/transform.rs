//!
//! Rotate and move objects.
//
/// Defines the rotation of the object.
pub trait Rotate<Origin, Axis> {
    ///
    /// Consumes `self` and returns a new rotated instance, where
    /// - `origin` - rotation pivot,
    /// - `axis` - axis to rotate around,
    /// - `rad` - angle in radians.
    fn rotate(self, origin: Origin, axis: Axis, rad: f64) -> Self;
}
///
/// Moving in space.
pub trait Translate<Dir> {
    ///
    /// Consumes `self` and returns a new translated instance moved to `dir`.
    fn translate(self, dir: Dir) -> Self;
}
