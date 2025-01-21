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
///
/// Algorithm to volume together three sets of the objects.
pub trait AlgoMakerVolume<A, B, C> {
    type Error;
    //
    //
    fn build<'a>(
        a: impl IntoIterator<Item = &'a A>,
        b: impl IntoIterator<Item = &'a B>,
        c: impl IntoIterator<Item = &'a C>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        A: 'a,
        B: 'a,
        C: 'a;
}
