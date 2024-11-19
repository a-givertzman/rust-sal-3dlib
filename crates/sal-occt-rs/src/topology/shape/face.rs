use super::*;
use crate::gmath::Vector;
use glam::DVec3;
use sal_3dlib_core::{
    ops::transform::{Rotate, Translate},
    props::{Area, Center},
};
//
//
impl Area for Face {
    fn area(&self) -> f64 {
        self.0.surface_area()
    }
}
//
//
impl Center for Face {
    type Output = Vertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        Vertex(primitives::Vertex::new(point))
    }
}
//
//
impl TryFrom<&Wire> for Face {
    type Error = String;
    //
    //
    fn try_from(wire: &Wire) -> Result<Self, Self::Error> {
        Ok(Self(primitives::Face::from_wire(&wire.0)))
    }
}
//
//
impl Rotate<Vertex, Vector> for Face {
    fn rotated(self, _origin: Vertex, axis: Vector, rad: f64) -> Self {
        Self(self.0.rotated(DVec3::from_array(*axis.0), rad))
    }
}
//
//
impl Translate<Vector> for Face {
    fn translated(self, dir: Vector) -> Self {
        Self(self.0.translated(DVec3::from_array(*dir.0)))
    }
}
