use crate::{
    gmath::Point,
    props::{Attributes, Dist},
};
///
/// Zero-size shape corresponding to a point in the geometry.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `V`,
/// - an optional attribute.
pub struct Vertex<const N: usize, V, T> {
    pub(super) inner: V,
    pub(super) attrs: Option<Attributes<T>>,
}
//
//
impl<const N: usize, V, T> Vertex<N, V, T> {
    ///
    /// Returns a numeric representation.
    ///
    /// # Examples
    /// Get numeric representation of the origin:
    /// ```ignore
    /// let origin = Vertex::origin();
    /// assert_eq!([0.0, 0.0, 0.0], origin.point());
    /// ```
    pub fn point(&self) -> [f64; N]
    where
        Point<N>: for<'a> From<&'a V>,
    {
        *Point::from(&self.inner)
    }
    ///
    /// Creates a new instance from coordinates.
    pub fn new(val: impl Into<[f64; N]>) -> Self
    where
        V: From<Point<N>>,
    {
        Self {
            inner: V::from(Point(val.into())),
            attrs: None,
        }
    }
    ///
    /// Returns a zeroed instance.
    pub fn origin() -> Self
    where
        V: From<Point<N>>,
    {
        Self {
            inner: V::from(Point([0.0; N])),
            attrs: None,
        }
    }
}
//
//
impl<const N: usize, V, T> Dist<Vertex<N, V, T>> for Vertex<N, V, T>
where
    V: Dist<V>,
{
    fn dist(&self, other: &Vertex<N, V, T>) -> f64 {
        self.inner.dist(&other.inner)
    }
}
//
//
impl<const N: usize, V, T> From<(V, Attributes<T>)> for Vertex<N, V, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((vertex, attrs): (V, Attributes<T>)) -> Self {
        Self {
            inner: vertex,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, V, T> Clone for Vertex<N, V, T>
where
    V: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
