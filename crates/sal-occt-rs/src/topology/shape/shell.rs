//!
//! Shell implementation in terms of OCCT.
//!
//! It provides the final object - [Shell] - and its related trait implementations.
//
use super::{face::OcctFace, solid::OcctSolid, vertex::OcctVertex};
use crate::ops::boolean::volume::VolumeConf;
use anyhow::Result;
use opencascade::primitives;
use sal_3dlib_core::{ops::boolean::volume::Volume, props::Center, topology::shape::shell};
///
/// Collection of faces connected by some edges of their wire boundaries.
///
/// For internal use only. It provides low-level implementation for [Shell].
#[derive(Clone)]
pub struct OcctShell(pub(crate) primitives::Shell);
//
//
impl Volume<OcctFace, VolumeConf, Result<OcctSolid>> for OcctShell {
    fn volume(&self, face: &OcctFace, _: VolumeConf) -> Result<OcctSolid> {
        Ok(OcctSolid(self.0.volume(&face.0)?))
    }
}
//
//
impl Center for OcctShell {
    type Output = OcctVertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        OcctVertex(primitives::Vertex::new(point))
    }
}
///
/// Collection of faces connected by some edges of their wire boundaries.
pub type Shell<T> = shell::Shell<3, OcctShell, T>;
