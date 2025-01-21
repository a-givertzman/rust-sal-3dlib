use super::*;
use crate::{
    gmath::Vector,
    ops::transform::{Rotate, Translate},
    props::{Area, Center},
};
//
//
impl<const N: usize, F, V, T> Center for Face<N, F, T>
where
    F: Center<Output = V>,
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
impl<const N: usize, F: Area, T> Area for Face<N, F, T> {
    fn area(&self) -> f64 {
        self.inner.area()
    }
}
//
//
impl<const N: usize, W, F, E, T> TryFrom<&Wire<N, W, T>> for Face<N, F, T>
where
    F: for<'a> TryFrom<&'a W, Error = E>,
{
    type Error = E;
    //
    //
    fn try_from(wire: &Wire<N, W, T>) -> Result<Self, Self::Error> {
        F::try_from(&wire.inner).map(|face| Self {
            inner: face,
            attrs: None,
        })
    }
}
//
//
impl<const N: usize, F, V, A, T> Rotate<Vertex<N, V, T>, A> for Face<N, F, T>
where
    F: Rotate<V, A>,
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
impl<const N: usize, F, D, T> Translate<D> for Face<N, F, T>
where
    F: Translate<D>,
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
impl<const N: usize, F, T> From<(F, Attributes<T>)> for Face<N, F, T> {
    fn from((face, attrs): (F, Attributes<T>)) -> Self {
        Self {
            inner: face,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, F, T> Clone for Face<N, F, T>
where
    F: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            attrs: self.attrs.clone(),
        }
    }
}
