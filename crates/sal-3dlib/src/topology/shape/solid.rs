use super::*;
use crate::{
    ops::boolean::{Intersect, OpConf},
    props::Volume,
};
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
    fn from((solid, attrs): (S, Attributes<T>)) -> Self {
        Self {
            inner: solid,
            attrs: Some(attrs),
        }
    }
}
