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
