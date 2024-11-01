use super::*;
use crate::{
    gmath::Vector,
    ops::{
        boolean::volume::{Volume, VolumeConf},
        transform::{Rotate, Translate},
    },
    props::Metadata,
};
//
//
impl<const N: usize, L, T> Metadata<T> for Shell<N, L, T> {
    fn attrs(&self) -> Option<&Attrs<T>> {
        self.attrs.as_ref()
    }
    //
    //
    fn attrs_mut(&mut self) -> Option<&mut Attrs<T>> {
        self.attrs.as_mut()
    }
}
//
//
impl<const N: usize, L, V, A, T> Rotate<Vertex<N, V, T>, A> for Shell<N, L, T>
where
    L: Rotate<V, A>,
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
impl<const N: usize, L, D, T> Translate<D> for Shell<N, L, T>
where
    L: Translate<D>,
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
impl<const N: usize, L, F, C, D, T> Volume<Face<N, F, T>, C> for Shell<N, L, T>
where
    L: Volume<F, C, Output = Option<D>>,
    VolumeConf: From<C>,
{
    type Output = Option<Solid<N, D, T>>;
    //
    //
    fn volume(&self, face: &Face<N, F, T>, conf: C) -> Self::Output {
        self.inner.volume(&face.inner, conf).map(|solid| Solid {
            inner: solid,
            attrs: None,
        })
    }
}
