use super::{face::Face, solid::Solid, vertex::Vertex};
use crate::{
    gmath::Vector,
    ops::{
        boolean::volume::{Volume, VolumeConf},
        transform::{Rotate, Translate},
    },
    props::{Attributes, Center, Metadata},
};
///
/// Set of faces connected by some edges of their wire boundaries.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `S`,
/// - an optional attribute.
pub struct Shell<const N: usize, S, T> {
    pub(super) inner: S,
    pub(super) attrs: Attributes<T>,
}
//
//
impl<const N: usize, S, T> Metadata<T> for Shell<N, S, T> {
    //
    //
    fn attrs(&self) -> &Attributes<T> {
        &self.attrs
    }
    //
    //
    fn attrs_mut(&mut self) -> &mut Attributes<T> {
        &mut self.attrs
    }
}
//
//
impl<const N: usize, S, V, A, T> Rotate<Vertex<N, V, T>, A> for Shell<N, S, T>
where
    S: Rotate<V, A>,
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
impl<const N: usize, S, D, T> Translate<D> for Shell<N, S, T>
where
    S: Translate<D>,
    D: Into<Vector<N>>,
{
    fn translate(self, dir: D) -> Self {
        Self {
            inner: self.inner.translate(dir),
            attrs: self.attrs,
        }
    }
}
//
//
impl<const N: usize, L, F, C, D, T, E> Volume<Face<N, F, T>, C, Result<Solid<N, D, T>, E>>
    for Shell<N, L, T>
where
    L: Volume<F, C, Result<D, E>>,
    VolumeConf: From<C>,
{
    fn volume(&self, face: &Face<N, F, T>, conf: C) -> Result<Solid<N, D, T>, E> {
        self.inner.volume(&face.inner, conf).map(|solid| Solid {
            inner: solid,
            attrs: None,
        })
    }
}
//
//
impl<const N: usize, S, T> From<(S, Attributes<T>)> for Shell<N, S, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((shell, attrs): (S, Attributes<T>)) -> Self {
        Self {
            inner: shell,
            attrs: attrs,
        }
    }
}
//
//
impl<const N: usize, S, V, T> Center for Shell<N, S, T>
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
//
//
impl<const N: usize, S, T> Clone for Shell<N, S, T>
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
