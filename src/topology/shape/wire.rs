use super::*;
//
//
pub trait Polygon<V> {
    fn polygon(vxs: impl IntoIterator<Item = V>, closed: bool) -> Self;
}
//
//
impl<const N: usize, W, V, T> Polygon<Vertex<N, V, T>> for Wire<N, W, T>
where
    W: Polygon<V>,
{
    ///
    /// Builds polygonal wires
    fn polygon(vxs: impl IntoIterator<Item = Vertex<N, V, T>>, closed: bool) -> Wire<N, W, T> {
        Self {
            inner: W::polygon(vxs.into_iter().map(|v| v.inner), closed),
            attrs: None,
        }
    }
}
