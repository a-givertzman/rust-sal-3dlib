use super::*;
use crate::{gmath::Point, props::Dist};
//
//
impl<const N: usize, V, T> Vertex<N, V, T> {
    ///
    /// Returns numeric representation
    pub fn point(&self) -> [f64; N]
    where
        Point<N>: for<'a> From<&'a V>,
    {
        *Point::from(&self.inner)
    }
    //
    //
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
    /// Returns zeroed Vertex
    /// ```
    /// let v = Vertex::<3, [f64; 3]>::origin();
    /// let p = v.point();
    /// assert_eq!(p, [0.0; 3]);
    /// ```
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
