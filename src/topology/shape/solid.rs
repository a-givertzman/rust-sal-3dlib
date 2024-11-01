use super::*;
use crate::{
    ops::boolean::{Intersect, OpConf},
    props::Volume,
};
//
//
impl<const N: usize, D: Volume, T> Volume for Solid<N, D, T> {
    fn volume(&self) -> f64 {
        self.inner.volume()
    }
}
//
//
impl<const N: usize, D, F, C, T> Intersect<Face<N, F, T>, C> for Solid<N, D, T>
where
    D: Intersect<F, C, Output = F>,
    OpConf: From<C>,
{
    type Output = Face<N, F, T>;
    //
    //
    fn intersect(&self, face: &Face<N, F, T>, conf: C) -> Self::Output {
        Face {
            inner: self.inner.intersect(&face.inner, conf),
            attrs: None,
        }
    }
}
