//!
//! Vertex implementation in terms of OCCT.
//!
//! It provides the final object - [Vertex] - and its related trait implementations.
//
use crate::gmath::point::Point;
use glam::DVec3;
use opencascade::primitives;
use sal_3dlib_core::{props::Dist, topology};
///
/// Zero-dimensional shape corresponding to a point in geometry.
///
/// For internal use only. It provides low-level implementation for [Vertex].
#[derive(Clone)]
pub struct OcctVertex(pub(crate) primitives::Vertex);
//
//
impl From<Point> for OcctVertex {
    fn from(point: Point) -> Self {
        Self(primitives::Vertex::new(DVec3::from_array(*point)))
    }
}
//
//
impl Dist<OcctVertex> for OcctVertex {
    fn dist(&self, other: &OcctVertex) -> f64 {
        self.0.dist(&other.0)
    }
}
///
/// Zero-dimensional shape corresponding to a point in geometry.
pub type Vertex<T> = topology::Vertex<3, OcctVertex, T>;
