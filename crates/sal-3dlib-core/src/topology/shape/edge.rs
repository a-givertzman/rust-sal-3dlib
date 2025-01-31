use crate::props::Attributes;
///
/// One-dimensional shape corresponding to a curve.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `E`,
/// - an optional attribute.
pub struct Edge<const N: usize, E, T> {
    inner: E,
    attrs: Option<Attributes<T>>,
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
