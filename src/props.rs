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
pub trait Metadata {
    fn attrs(&self) -> Option<&Attrs>;
}
///
/// User defined attributes
pub struct Attrs {
    pub tag: String,
}
