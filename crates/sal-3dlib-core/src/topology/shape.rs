//! Main topological entities.
//
#![allow(dead_code)]
//
pub mod compound;
mod edge;
mod face;
mod shell;
mod solid;
mod vertex;
mod wire;
//
use crate::props::Attributes;
///
/// Zero-size shape corresponding to a point in the geometry.
pub struct Vertex<const N: usize, V, T> {
    inner: V,
    attrs: Option<Attributes<T>>,
}
///
/// One-dimensional shape corresponding to a curve.
pub struct Edge<const N: usize, E, T> {
    inner: E,
    attrs: Option<Attributes<T>>,
}
///
/// Sequence of edges connected by their vertices.
pub struct Wire<const N: usize, W, T> {
    inner: W,
    attrs: Option<Attributes<T>>,
}
///
/// Part of a surface (e. g. a plane in 2D geometry) bounded by a closed wire.
pub struct Face<const N: usize, F, T> {
    pub inner: F,
    attrs: Option<Attributes<T>>,
}
///
/// Set of faces connected by some edges of their wire boundaries.
pub struct Shell<const N: usize, S, T> {
    inner: S,
    attrs: Option<Attributes<T>>,
}
///
/// Part of the N-dimensional space bounded by shells.
pub struct Solid<const N: usize, S, T> {
    pub inner: S,
    attrs: Option<Attributes<T>>,
}
///
/// Group of any of main entities.
pub struct Compound<const N: usize, C, T> {
    pub inner: C,
    attrs: Option<Attributes<T>>,
}
