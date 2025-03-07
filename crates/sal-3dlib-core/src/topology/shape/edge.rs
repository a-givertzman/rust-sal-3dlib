use super::vertex::Vertex;
use crate::{
    gmath::Vector,
    ops::transform::{Rotate, Translate},
    props::{Attributes, Center, Length},
};
///
/// One-dimensional shape corresponding to a curve.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `E`,
/// - an optional attribute.
pub struct Edge<const N: usize, E, T> {
    pub(crate) inner: E,
    pub(crate) attrs: Option<Attributes<T>>,
}
//
//
impl<const N: usize, E, T> From<(E, Attributes<T>)> for Edge<N, E, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((edge, attrs): (E, Attributes<T>)) -> Self {
        Self {
            inner: edge,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, E, T> Clone for Edge<N, E, T>
where
    E: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
//
//
impl<const N: usize, E, V, T> Center for Edge<N, E, T>
where
    E: Center<Output = V>,
{
    type Output = Vertex<N, V, T>;
    //
    //
    fn center(&self) -> Self::Output {
        Vertex::<N, V, T> {
            inner: self.inner.center(),
            attrs: None,
        }
    }
}
///
/// Edge with direction.
pub trait Direction<Dir> {
    ///
    /// Returns the direction.
    fn dir(&self) -> Dir;
}
//
//
impl<const N: usize, E, T, V> Direction<V> for Edge<N, E, T>
where
    E: Direction<V>,
    V: Into<Vector<N>>,
{
    fn dir(&self) -> V {
        self.inner.dir()
    }
}
//
//
impl<const N: usize, E, T> Length for Edge<N, E, T>
where
    E: Length,
{
    fn len(&self) -> f64 {
        self.inner.len()
    }
}
//
//
impl<const N: usize, E, V, A, T> Rotate<Vertex<N, V, T>, A> for Edge<N, E, T>
where
    E: Rotate<V, A>,
    A: Into<Vector<N>>,
{
    fn rotate(self, origin: Vertex<N, V, T>, axis: A, angle: f64) -> Self {
        let origin = origin.inner;
        Self {
            inner: self.inner.rotate(origin, axis, angle),
            attrs: self.attrs,
        }
    }
}
//
//
impl<const N: usize, E, D, T> Translate<D> for Edge<N, E, T>
where
    E: Translate<D>,
    D: Into<Vector<N>>,
{
    fn translate(self, dir: D) -> Self {
        Self {
            inner: self.inner.translate(dir),
            attrs: self.attrs,
        }
    }
}
