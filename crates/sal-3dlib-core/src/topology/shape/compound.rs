use super::{edge::Edge, face::Face, shell::Shell, solid::Solid, vertex::Vertex};
use crate::{
    ops::boolean::volume::AlgoMakerVolume,
    props::{Attributes, Center},
};
///
/// Group of any of main entities.
///
/// It depends on:
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel - `C`,
/// - an optional attribute.
pub struct Compound<const N: usize, C, T> {
    pub(super) inner: C,
    pub(super) attrs: Option<Attributes<T>>,
}
//
//
impl<const N: usize, C, T> From<(C, Attributes<T>)> for Compound<N, C, T> {
    ///
    /// Creates an instance from its inner representation and given attribute.
    fn from((compound, attrs): (C, Attributes<T>)) -> Self {
        Self {
            inner: compound,
            attrs: Some(attrs),
        }
    }
}
//
//
impl<const N: usize, C, V, T> Center for Compound<N, C, T>
where
    C: Center<Output = V>,
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
impl<const N: usize, C, T> Clone for Compound<N, C, T>
where
    C: Clone,
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
impl<const N: usize, C, E, F, L, D, T>
    AlgoMakerVolume<Face<N, F, T>, Shell<N, L, T>, Solid<N, D, T>> for Compound<N, C, T>
where
    C: AlgoMakerVolume<F, L, D, Error = E>,
{
    type Error = E;
    //
    //
    fn build<'a>(
        fs: impl IntoIterator<Item = &'a Face<N, F, T>>,
        ls: impl IntoIterator<Item = &'a Shell<N, L, T>>,
        ds: impl IntoIterator<Item = &'a Solid<N, D, T>>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Face<N, F, T>: 'a,
        Shell<N, L, T>: 'a,
        Solid<N, D, T>: 'a,
    {
        C::build(
            fs.into_iter().map(|face| &face.inner),
            ls.into_iter().map(|shell| &shell.inner),
            ds.into_iter().map(|solid| &solid.inner),
        )
        .map(|compound| Self {
            inner: compound,
            attrs: None,
        })
    }
}
///
/// Object, which can be represented as solids.
pub trait Solids<T> {
    ///
    /// Returns the iterator over object solids.
    fn solids(&self) -> impl IntoIterator<Item = T>;
}
//
//
impl<const N: usize, S, C, T> Solids<Solid<N, S, T>> for Compound<N, C, T>
where
    C: Solids<S>,
{
    fn solids(&self) -> impl IntoIterator<Item = Solid<N, S, T>> {
        self.inner.solids().into_iter().map(|solid| Solid {
            inner: solid,
            attrs: None,
        })
    }
}
//
//
pub trait Edges<T> {
    fn edges(&self) -> impl IntoIterator<Item = T>;
}
//
//
impl<const N: usize, E, C, T> Edges<Edge<N, E, T>> for Compound<N, C, T>
where
    C: Edges<E>,
{
    fn edges(&self) -> impl IntoIterator<Item = Edge<N, E, T>> {
        self.inner.edges().into_iter().map(|edge| Edge {
            inner: edge,
            attrs: None,
        })
    }
}
