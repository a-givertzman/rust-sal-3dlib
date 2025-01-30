//!
//! Solid implementation in terms of OCCT.
//!
//! It provides the final object - [Solid] - and its related trait implementations.
//
use super::{compound::OcctCompound, face::OcctFace, vertex::OcctVertex};
use crate::ops::boolean::OpConf;
use opencascade::primitives::{self, IntoShape};
use sal_3dlib_core::{
    ops::boolean::Intersect,
    props::{Center, Volume},
    topology,
};
///
/// Part of space limited by shells.
///
/// For internal use only. It provides low-level implementation for [Solid].
#[derive(Clone)]
pub struct OcctSolid(pub(crate) primitives::Solid);
//
//
impl Volume for OcctSolid {
    fn volume(&self) -> f64 {
        self.0.volume()
    }
}
//
//
impl Intersect<OcctFace, OpConf, OcctCompound> for OcctSolid {
    fn intersect(&self, face: &OcctFace, conf: OpConf) -> OcctCompound {
        let this = self.0.as_ref().into_shape();
        let other = face.0.as_ref().into_shape();
        OcctCompound(this.intersect_with(&other, conf.parallel))
    }
}
//
//
impl Center for OcctSolid {
    type Output = OcctVertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        OcctVertex(primitives::Vertex::new(point))
    }
}
///
/// Part of space limited by shells.
pub type Solid<T> = topology::Solid<3, OcctSolid, T>;
