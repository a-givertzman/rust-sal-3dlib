//!
//! Face implementation in terms of OCCT.
//!
//! It provides the final object - [Face] - and its related trait implementations.
//
use super::{vertex::OcctVertex, wire::OcctWire};
use crate::gmath::vector::Vector;
use glam::DVec3;
use opencascade::primitives;
use sal_3dlib_core::{
    ops::transform::{Rotate, Translate},
    props::{Area, Center},
    topology::shape::face,
};
///
/// Part of a surface bounded by a closed wire.
///
/// For internal use only. It provides low-level implementation for [Face].
#[derive(Clone)]
pub struct OcctFace(pub(crate) primitives::Face);
//
//
impl Area for OcctFace {
    fn area(&self) -> f64 {
        self.0.surface_area()
    }
}
//
//
impl Center for OcctFace {
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
impl TryFrom<&OcctWire> for OcctFace {
    type Error = String;
    //
    //
    fn try_from(wire: &OcctWire) -> Result<Self, Self::Error> {
        Ok(Self(primitives::Face::from_wire(&wire.0)))
    }
}
//
//
impl Rotate<OcctVertex, Vector> for OcctFace {
    fn rotate(self, origin: OcctVertex, axis: Vector, rad: f64) -> Self {
        Self(self.0.rotated(origin.0, DVec3::from_array(*axis.0), rad))
    }
}
//
//
impl Translate<Vector> for OcctFace {
    fn translate(self, dir: Vector) -> Self {
        Self(self.0.translated(DVec3::from_array(*dir.0)))
    }
}
///
/// Part of a surface bounded by a closed wire.
pub type Face<T> = face::Face<3, OcctFace, T>;
