use super::*;
use crate::{ops::Solidify, props::Center};
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
//
//
impl<const N: usize, C, E, F, L, D, T> Solidify<Face<N, F, T>, Shell<N, L, T>, Solid<N, D, T>>
    for Compound<N, C, T>
where
    C: Solidify<F, L, D, Error = E>,
{
    type Error = E;
    //
    //
    fn solidify<'a>(
        fs: impl IntoIterator<Item = &'a Face<N, F, T>>,
        ls: impl IntoIterator<Item = &'a Shell<N, L, T>>,
        ds: impl IntoIterator<Item = &'a Solid<N, D, T>>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Face<N, F, T>: 'a,
        Shell<N, L, T>: 'a,
        Solid<N, D, T>: 'a,
    {
        C::solidify(
            fs.into_iter().map(|face| &face.inner),
            ls.into_iter().map(|shell| &shell.inner),
            ds.into_iter().map(|solid| &solid.inner),
        )
        .map(|compound| Self {
            inner: compound,
            attrs: None,
        })
    }
}
