//! Defining the [bounding box](https://en.wikipedia.org/wiki/Minimum_bounding_box).
///
/// Create a bounding box.
pub trait Bound<T> {
    ///
    /// Creates a new instance.
    fn bounding_box(&self) -> BoundingBox<T>;
}
///
/// Holds the minimum and maximum points of the bounding box.
pub struct BoundingBox<T>(T, T);
//
//
impl<T> BoundingBox<T> {
    ///
    /// Contructor without equality check.
    #[allow(unused)]
    pub(crate) fn new_unchecked(min: T, max: T) -> Self {
        Self(min, max)
    }
    ///
    /// Returns a reference to the minimum point.
    pub fn min(&self) -> &T {
        &self.0
    }
    ///
    /// Returns a reference to the maximum point.
    pub fn max(&self) -> &T {
        &self.1
    }
}
