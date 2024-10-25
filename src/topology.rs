mod compound;
mod shape;
//
use crate::props::Attrs;
use indexmap::IndexMap;
pub use shape::*;
///
/// Abstract topological data structure describes a basic entity
pub enum Shape<const N: usize, V, E, W, F, L, D> {
    Vertex(Vertex<N, V>),
    Edge(Edge<N, E>),
    Wire(Wire<N, W>),
    Face(Face<N, F>),
    Shell(Shell<N, L>),
    Solid(Solid<N, D>),
    Shape(Box<Shape<N, V, E, W, F, L, D>>),
    Compound(Compound<N, V, E, W, F, L, D>),
}
///
/// A group of any of the shapes
pub struct Compound<const N: usize, V, E, W, F, L, D> {
    shapes: IndexMap<String, Shape<N, V, E, W, F, L, D>>,
    attrs: Option<Attrs>,
}
