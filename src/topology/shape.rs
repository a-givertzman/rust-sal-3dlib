mod edge;
mod face;
mod shell;
mod vertex;
//
use crate::props::Attrs;
///
/// A zero-dimensional shape corresponding to a point in geometry
pub struct Vertex<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
///
/// A single dimensional shape corresponding to a curve
pub struct Edge<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
///
/// A sequence of edges connected by their vertices
pub struct Wire<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
///
/// Part of a plane (in 2D geometry) or a surface (in 3D geometry) bounded by a closed wire
pub struct Face<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
///
/// A set of faces connected by some of the edges of their wire boundaries
pub struct Shell<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
///
/// A part of 3D space bounded by shells
pub struct Solid<const N: usize, T> {
    inner: T,
    attrs: Option<Attrs>,
}
