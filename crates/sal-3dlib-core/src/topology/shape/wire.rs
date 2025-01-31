use super::vertex::Vertex;
use crate::{ops::Polygon, props::Attributes};
///
/// Sequence of edges connected by their vertices.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `W`,
/// - an optional attribute.
pub struct Wire<const N: usize, W, T> {
    pub(super) inner: W,
    pub(super) attrs: Option<Attributes<T>>,
}
//
//
impl<const N: usize, W, E, V, T> Polygon<Vertex<N, V, T>> for Wire<N, W, T>
where
    W: Polygon<V, Error = E>,
{
    type Error = E;
    //
    //
    fn polygon(
        vs: impl IntoIterator<Item = Vertex<N, V, T>>,
        closed: bool,
    ) -> Result<Wire<N, W, T>, Self::Error> {
        W::polygon(vs.into_iter().map(|v| v.inner), closed).map(|wire| Self {
            inner: wire,
            attrs: None,
        })
    }
}
//
//
impl<const N: usize, W, T> From<(W, Attributes<T>)> for Wire<N, W, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((wire, attrs): (W, Attributes<T>)) -> Self {
        Self {
            inner: wire,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, W, T> Clone for Wire<N, W, T>
where
    W: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
