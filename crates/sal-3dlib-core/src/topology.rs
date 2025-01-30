//!
//! Topology in N-dimensional space.
//
pub(crate) mod shape;
//
pub use shape::*;
///
/// Abstract topological data structure describes a basic entity.
pub enum Shape<const N: usize, V, E, W, F, L, D, C, T> {
    ///
    /// Zero-dimensional shape corresponding to a point in geometry.
    Vertex(Vertex<N, V, T>),
    ///
    /// Shape corresponding to a curve, and bound by a vertex at each extremity.
    Edge(Edge<N, E, T>),
    ///
    /// Sequence of edges connected by their vertices.
    Wire(Wire<N, W, T>),
    ///
    /// Part of a surface bounded by a closed wire.
    Face(Face<N, F, T>),
    ///
    /// Collection of faces connected by some edges of their wire boundaries.
    Shell(Shell<N, L, T>),
    ///
    /// Part of space limited by shells.
    Solid(Solid<N, D, T>),
    ///
    /// Group of any type of topological objects.
    Compound(Compound<N, C, T>),
}
//
//
impl<const N: usize, V, E, W, F, L, D, C, T> Clone for Shape<N, V, E, W, F, L, D, C, T>
where
    V: Clone,
    E: Clone,
    W: Clone,
    F: Clone,
    L: Clone,
    D: Clone,
    C: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Shape::Vertex(v) => Self::Vertex(v.clone()),
            Shape::Edge(e) => Self::Edge(e.clone()),
            Shape::Wire(w) => Self::Wire(w.clone()),
            Shape::Face(f) => Self::Face(f.clone()),
            Shape::Shell(s) => Self::Shell(s.clone()),
            Shape::Solid(s) => Self::Solid(s.clone()),
            Shape::Compound(c) => Self::Compound(c.clone()),
        }
    }
}
