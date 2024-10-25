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
impl<const N: usize, L> Metadata for Shell<N, L> {
    fn attrs(&self) -> Option<&Attrs> {
        self.attrs.as_ref()
    }
}
//
//
impl<const N: usize, L, V, A> Rotate<Vertex<N, V>, A> for Shell<N, L>
where
    L: Rotate<V, A>,
    A: Into<Vector<N>>,
{
    fn rotate(self, origin: Vertex<N, V>, axis: A, angle: f64) -> Self {
        let origin = origin.inner;
        Self {
            inner: self.inner.rotate(origin, axis, angle),
            attrs: self.attrs,
        }
    }
}
//
//
impl<const N: usize, L, D> Translate<D> for Shell<N, L>
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
impl<const N: usize, L, F, C, D> Volume<&Face<N, F>, C> for Shell<N, L>
where
    L: for<'a> Volume<&'a F, C, Output = Option<D>>,
    VolumeConf: From<C>,
{
    type Output = Option<Solid<N, D>>;
    //
    //
    fn volume(&self, face: &Face<N, F>, conf: C) -> Self::Output {
        self.inner.volume(&face.inner, conf).map(|d| Solid {
            inner: d,
            attrs: None,
        })
    }
}
