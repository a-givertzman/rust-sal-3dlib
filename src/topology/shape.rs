mod face;
mod shell;
mod solid;
mod vertex;
mod wire;
//
use crate::props::Attrs;
///
/// A zero-dimensional shape corresponding to a point in geometry
pub struct Vertex<const N: usize, V, T> {
    inner: V,
    attrs: Option<Attrs<T>>,
}
///
/// A single dimensional shape corresponding to a curve
pub struct Edge<const N: usize, E, T> {
    inner: E,
    attrs: Option<Attrs<T>>,
}
///
/// A sequence of edges connected by their vertices
pub struct Wire<const N: usize, W, T> {
    inner: W,
    attrs: Option<Attrs<T>>,
}
///
/// Part of a plane (in 2D geometry) or a surface (in 3D geometry) bounded by a closed wire
pub struct Face<const N: usize, F, T> {
    inner: F,
    attrs: Option<Attrs<T>>,
}
///
/// A set of faces connected by some of the edges of their wire boundaries
pub struct Shell<const N: usize, L, T> {
    inner: L,
    attrs: Option<Attrs<T>>,
}
///
/// A part of 3D space bounded by shells
pub struct Solid<const N: usize, D, T> {
    inner: D,
    attrs: Option<Attrs<T>>,
}
