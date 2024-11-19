use super::*;
use crate::gmath::Point;
use glam::DVec3;
use sal_3dlib_core::props::Dist;
//
//
impl From<Point> for Vertex {
    fn from(point: Point) -> Self {
        Self(primitives::Vertex::new(DVec3::from_array(*point)))
    }
}
//
//
impl Dist<Vertex> for Vertex {
    fn dist(&self, other: &Vertex) -> f64 {
        self.0.dist(&other.0)
    }
}
