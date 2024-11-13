//! Defines a topology in N-dimensional space.
//
mod shape;
//
pub use shape::*;
///
/// Abstract topological structure describes general entity.
pub enum Shape<const N: usize, V, E, W, F, L, D, C, T> {
    Vertex(Vertex<N, V, T>),
    Edge(Edge<N, E, T>),
    Wire(Wire<N, W, T>),
    Face(Face<N, F, T>),
    Shell(Shell<N, L, T>),
    Solid(Solid<N, D, T>),
    Compound(Compound<N, C, T>),
    Shape(Box<Self>),
}
