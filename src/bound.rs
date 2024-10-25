/// The trait for defining the bounding box
pub trait Bound<T> {
    fn bounding_box(&self) -> BoundingBox<T>;
}
//
//
pub struct BoundingBox<T>(T, T);
//
//
impl<T> BoundingBox<T> {
    ///
    /// Contructor without equality check
    pub(crate) fn new_unchecked(min: T, max: T) -> Self {
        Self(min, max)
    }
    ///
    /// Returns the reference to the minimum point
    pub fn min(&self) -> &T {
        &self.0
    }
    ///
    /// Returns the reference to the maximum point
    pub fn max(&self) -> &T {
        &self.1
    }
}
