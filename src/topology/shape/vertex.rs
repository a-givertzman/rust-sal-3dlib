use super::*;
use crate::{
    gmath::Point,
    ops::boolean::{Intersect, OpConf},
    props::{Dist, Metadata},
};
//
//
impl<const N: usize, T> Vertex<N, T> {
    pub fn point(&self) -> [f64; N]
    where
        Point<N>: for<'a> From<&'a T>,
    {
        *Point::from(&self.inner)
    }

    pub fn new(val: impl Into<[f64; N]>) -> Self
    where
        T: From<Point<N>>,
    {
        Self {
            inner: T::from(Point(val.into())),
            attrs: None,
        }
    }

    pub fn origin() -> Self
    where
        T: From<Point<N>>,
    {
        Self {
            inner: T::from(Point([0.0; N])),
            attrs: None,
        }
    }
}

impl<const N: usize, T> From<T> for Vertex<N, T> {
    fn from(val: T) -> Self {
        Self {
            inner: val,
            attrs: None,
        }
    }
}

impl<const N: usize, V, C> Intersect<Vertex<N, V>, C> for Vertex<N, V>
where
    V: Intersect<V, C, Output = Option<V>>,
    OpConf: From<C>,
{
    type Output = Option<Vertex<N, V>>;

    fn intersect(&self, vertex: &Vertex<N, V>, conf: C) -> Self::Output {
        self.inner.intersect(&vertex.inner, conf).map(|v| Self {
            inner: v,
            attrs: None,
        })
    }
}

impl<const N: usize, T> Dist<Vertex<N, T>> for Vertex<N, T>
where
    T: Dist<T>,
{
    fn dist(&self, other: &Vertex<N, T>) -> f64 {
        self.inner.dist(&other.inner)
    }
}

impl<const N: usize, T> Metadata for Vertex<N, T> {
    fn attrs(&self) -> Option<&Attrs> {
        self.attrs.as_ref()
    }
}
