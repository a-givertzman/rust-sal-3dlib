//!
//! Edge implementation in terms of OCCT.
//!
//! It provides the final object - [Edge] - and its related trait implementations.
use super::vertex::{OcctVertex, Vertex};
use crate::gmath::{point::Point, vector::Vector};
use glam::DVec3;
use opencascade::primitives;
pub use sal_3dlib_core::topology::shape::edge::Direction;
use sal_3dlib_core::{
    ops::transform,
    props::{Center, Length},
    topology::shape::edge,
};
///
/// Shape corresponding to a curve, and bound by a vertex at each extremity.
///
/// For internal use only. It provides low-level implementation for [Edge].
#[allow(dead_code)]
#[derive(Clone)]
pub struct OcctEdge(pub(crate) primitives::Edge);
//
//
impl Center for OcctEdge {
    type Output = OcctVertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        OcctVertex(primitives::Vertex::new(point))
    }
}
//
//
impl Direction<Vector> for OcctEdge {
    fn dir(&self) -> Vector {
        Vector::from([
            Point::from(self.0.start_point().to_array()),
            Point::from(self.0.end_point().to_array()),
        ])
    }
}
//
//
impl Length for OcctEdge {
    fn len(&self) -> f64 {
        let start = self.0.start_point();
        let end = self.0.end_point();
        start.distance(end)
    }
}
//
//
impl transform::Rotate<OcctVertex, Vector> for OcctEdge {
    fn rotate(self, origin: OcctVertex, axis: Vector, rad: f64) -> Self {
        Self(self.0.rotated(origin.0, DVec3::from_array(*axis.0), rad))
    }
}
//
//
impl transform::Translate<Vector> for OcctEdge {
    fn translate(self, dir: Vector) -> Self {
        Self(self.0.translated(DVec3::from_array(*dir.0)))
    }
}
///
/// Shape corresponding to a curve, and bound by a vertex at each extremity.
pub type Edge<T> = edge::Edge<3, OcctEdge, T>;
//
//
pub trait Rotate<T>: transform::Rotate<Vertex<T>, Vector> {
    fn rotate(self, origin: Vertex<T>, axis: Vector, rad: f64) -> Self
    where
        Self: Sized,
    {
        <Self as transform::Rotate<Vertex<T>, Vector>>::rotate(self, origin, axis, rad)
    }
}
//
//
pub trait Translate<T>: transform::Translate<Vector> {
    fn translate(self, dir: Vector) -> Self
    where
        Self: Sized,
    {
        <Self as transform::Translate<Vector>>::translate(self, dir)
    }
}
//
//
impl<T> Rotate<T> for Edge<T> {}
impl<T> Translate<T> for Edge<T> {}
