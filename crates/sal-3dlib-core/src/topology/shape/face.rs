use super::{compound::Compound, vertex::Vertex, wire::Wire};
use crate::{
    gmath::Vector,
    ops::{
        boolean::{Intersect, OpConf},
        transform::{Rotate, Translate},
        Project, Rectangle,
    },
    props::{Area, Attributes, Center, Normal},
};
///
/// Part of a surface bounded by a closed wire.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `F`,
/// - an optional attribute.
pub struct Face<const N: usize, F, T> {
    pub(super) inner: F,
    pub(super) attrs: Option<Attributes<T>>,
}
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
    ///
    /// Creates an instance from its inner representation and given attribute.
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
//
//
impl<const N: usize, V, D, F, T> Rectangle<Vertex<N, V, T>, D> for Face<N, F, T>
where
    F: Rectangle<V, D>,
    D: Into<Vector<N>>,
{
    fn rect(center: &Vertex<N, V, T>, normal: &D, height: f64, width: f64) -> Self {
        Self {
            inner: F::rect(&center.inner, normal, height, width),
            attrs: None,
        }
    }
}
//
//
impl<const N: usize, F, V, E, T> Project<Vertex<N, V, T>> for Face<N, F, T>
where
    F: Project<V, Error = E>,
{
    type Error = F::Error;
    //
    //
    fn project(&self, point: &Vertex<N, V, T>) -> Result<Vertex<N, V, T>, Self::Error> {
        self.inner.project(&point.inner).map(|vertex| Vertex {
            inner: vertex,
            attrs: None,
        })
    }
}
//
//
impl<const N: usize, F, V, D, T> Normal<Vertex<N, V, T>, D> for Face<N, F, T>
where
    F: Normal<V, D>,
    D: Into<Vector<N>>,
{
    fn normal_at(&self, point: &Vertex<N, V, T>) -> D {
        self.inner.normal_at(&point.inner)
    }
}
//
//
impl<const N: usize, F, C, D, T> Intersect<Self, C, Compound<N, D, T>> for Face<N, F, T>
where
    F: Intersect<F, C, D>,
    OpConf: From<C>,
{
    fn intersect(&self, rhs: &Self, conf: C) -> Compound<N, D, T> {
        Compound {
            inner: self.inner.intersect(&rhs.inner, conf),
            attrs: None,
        }
    }
}
