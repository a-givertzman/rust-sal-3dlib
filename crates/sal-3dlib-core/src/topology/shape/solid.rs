use super::{compound::Compound, face::Face, vertex::Vertex};
use crate::{
    ops::boolean::{Intersect, OpConf},
    props::{Attributes, Center, Volume},
};
///
/// Part of the N-dimensional space bounded by shells.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `S`,
/// - an optional attribute.
pub struct Solid<const N: usize, S, T> {
    pub(super) inner: S,
    pub(super) attrs: Option<Attributes<T>>,
}
//
//
impl<const N: usize, S: Volume, T> Volume for Solid<N, S, T> {
    fn volume(&self) -> f64 {
        self.inner.volume()
    }
}
//
//
impl<const N: usize, F, D, S, C, T> Intersect<Face<N, F, T>, C, Compound<N, D, T>>
    for Solid<N, S, T>
where
    S: Intersect<F, C, D>,
    OpConf: From<C>,
{
    fn intersect(&self, face: &Face<N, F, T>, conf: C) -> Compound<N, D, T> {
        Compound {
            inner: self.inner.intersect(&face.inner, conf),
            attrs: None,
        }
    }
}
//
//
impl<const N: usize, S, T> From<(S, Attributes<T>)> for Solid<N, S, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((solid, attrs): (S, Attributes<T>)) -> Self {
        Self {
            inner: solid,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, S, T> Clone for Solid<N, S, T>
where
    S: Clone,
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
impl<const N: usize, S, V, T> Center for Solid<N, S, T>
where
    S: Center<Output = V>,
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
