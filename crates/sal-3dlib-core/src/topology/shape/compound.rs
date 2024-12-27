use super::*;
use crate::props::Center;
//
//
impl<const N: usize, C, T> From<(C, Attributes<T>)> for Compound<N, C, T> {
    fn from((compound, attrs): (C, Attributes<T>)) -> Self {
        Self {
            inner: compound,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, C, V, T> Center for Compound<N, C, T>
where
    C: Center<Output = V>,
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
//
//
impl<const N: usize, C, T> Clone for Compound<N, C, T>
where
    C: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
