use super::*;
use crate::topology::shape::Vertex;
//
//
impl From<&Vertex> for Point {
    fn from(vertex: &Vertex) -> Self {
        let v = &vertex.0;
        Self::from([v.x(), v.y(), v.z()])
    }
}
