//!
//! Main topological entities.
//!
//! These entities allow accessing and manipulating data of objects
//! without dealing with their N-dimensional representations.
//
pub mod compound;
pub mod edge;
pub mod face;
pub mod shell;
pub mod solid;
pub mod vertex;
pub mod wire;
///
/// Abstract topological data structure describes a basic entity.
///
/// Each variant depends on
/// - the space dimension - `N`,
/// - the inner implementation specific to the kernel (`V`, `E`, etc.),
/// - an attribute of type `T` (related to [Attributes]).
///
/// [Attributes]: crate::props::Attributes
pub enum Shape<const N: usize, V, E, W, F, L, D, C, T> {
    ///
    /// Zero-dimensional shape corresponding to a point in geometry.
    Vertex(vertex::Vertex<N, V, T>),
    ///
    /// Shape corresponding to a curve, and bound by a vertex at each extremity.
    Edge(edge::Edge<N, E, T>),
    ///
    /// Sequence of edges connected by their vertices.
    Wire(wire::Wire<N, W, T>),
    ///
    /// Part of a surface bounded by a closed wire.
    Face(face::Face<N, F, T>),
    ///
    /// Collection of faces connected by some edges of their wire boundaries.
    Shell(shell::Shell<N, L, T>),
    ///
    /// Part of space limited by shells.
    Solid(solid::Solid<N, D, T>),
    ///
    /// Group of any type of topological objects.
    Compound(compound::Compound<N, C, T>),
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
