mod compound;
mod shape;
//
use crate::props::Attrs;
use indexmap::IndexMap;
pub use shape::*;
///
/// Abstract topological data structure describes a basic entity
pub enum Shape<const N: usize, V, E, W, F, L, D, T> {
    Vertex(Vertex<N, V, T>),
    Edge(Edge<N, E, T>),
    Wire(Wire<N, W, T>),
    Face(Face<N, F, T>),
    Shell(Shell<N, L, T>),
    Solid(Solid<N, D, T>),
    Shape(Box<Shape<N, V, E, W, F, L, D, T>>),
    Compound(Compound<N, V, E, W, F, L, D, T>),
}
///
/// A group of any of the shapes
#[allow(clippy::type_complexity)]
pub struct Compound<const N: usize, V, E, W, F, L, D, T> {
    shapes: IndexMap<String, Shape<N, V, E, W, F, L, D, T>>,
    attrs: Option<Attrs<T>>,
}
