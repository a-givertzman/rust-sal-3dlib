#[allow(clippy::len_without_is_empty)]
pub trait Length {
    fn len(&self) -> f64;
}
//
//
pub trait Area {
    fn area(&self) -> f64;
}
//
//
pub trait Volume {
    fn volume(&self) -> f64;
}
//
//
pub trait Center {
    type Output;
    //
    //
    fn center(&self) -> Self::Output;
}
//
//
pub trait Dist<T> {
    fn dist(&self, t: &T) -> f64;
}
///
/// Manages attached information
pub trait Metadata<T> {
    fn attrs(&self) -> Option<&Attrs<T>>;
    //
    //
    fn attrs_mut(&mut self) -> Option<&mut Attrs<T>>;
}
//
//
pub struct Attrs<T> {
    pub name: String,
    ///
    /// User defined metadata
    pub custom: T,
}
