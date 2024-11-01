use super::*;
use crate::props::{Area, Center};
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
