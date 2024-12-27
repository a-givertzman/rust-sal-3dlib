use super::*;
//
//
impl<const N: usize, E, T> From<(E, Attributes<T>)> for Edge<N, E, T> {
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
