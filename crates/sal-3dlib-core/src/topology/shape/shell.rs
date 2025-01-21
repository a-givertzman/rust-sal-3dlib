use super::*;
use crate::{
    gmath::Vector,
    ops::{
        boolean::volume::{Volume, VolumeConf},
        transform::{Rotate, Translate},
    },
    props::{Center, Metadata},
};
//
//
impl<const N: usize, S, T> Metadata<T> for Shell<N, S, T> {
    fn attrs(&self) -> Option<&Attributes<T>> {
        self.attrs.as_ref()
    }
    //
    //
    fn attrs_mut(&mut self) -> Option<&mut Attributes<T>> {
        self.attrs.as_mut()
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
    fn from((shell, attrs): (S, Attributes<T>)) -> Self {
        Self {
            inner: shell,
            attrs: Some(attrs),
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
