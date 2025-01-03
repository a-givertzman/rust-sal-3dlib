use super::*;
use crate::ops::Polygon;
//
//
impl<const N: usize, W, E, V, T> Polygon<Vertex<N, V, T>> for Wire<N, W, T>
where
    W: Polygon<V, Error = E>,
{
    type Error = E;
    ///
    /// Builds a polygonal wire.
    fn polygon(
        vxs: impl IntoIterator<Item = Vertex<N, V, T>>,
        closed: bool,
    ) -> Result<Wire<N, W, T>, Self::Error> {
        W::polygon(vxs.into_iter().map(|v| v.inner), closed).map(|wire| Self {
            inner: wire,
            attrs: None,
        })
    }
}
//
//
impl<const N: usize, W, T> From<(W, Attributes<T>)> for Wire<N, W, T> {
    fn from((wire, attrs): (W, Attributes<T>)) -> Self {
        Self {
            inner: wire,
            attrs: Some(attrs),
        }
    }
}
