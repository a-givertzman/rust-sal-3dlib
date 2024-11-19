use super::*;
use crate::ops::boolean::volume::VolumeConf;
use anyhow::Result;
use sal_3dlib::{ops::boolean::volume::Volume, props::Center};
//
//
impl Volume<Face, VolumeConf, Result<Solid>> for Shell {
    fn volume(&self, face: &Face, _: VolumeConf) -> Result<Solid> {
        Ok(Solid(self.0.volume(&face.0)?))
    }
}
//
//
impl Center for Shell {
    type Output = Vertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        Vertex(primitives::Vertex::new(point))
    }
}
