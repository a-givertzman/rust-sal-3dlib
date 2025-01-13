use super::*;
use crate::ops::boolean::OpConf;
use primitives::IntoShape;
use sal_3dlib_core::{
    ops::boolean::Intersect,
    props::{Center, Volume},
};
//
//
impl Volume for Solid {
    fn volume(&self) -> f64 {
        self.0.volume()
    }
}
//
//
impl Intersect<Face, OpConf, Compound> for Solid {
    fn intersect(&self, face: &Face, conf: OpConf) -> Compound {
        let this = self.0.as_ref().into_shape();
        let other = face.0.as_ref().into_shape();
        Compound(this.intersect_with(&other, conf.parallel))
    }
}
//
//
impl Center for Solid {
    type Output = Vertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        Vertex(primitives::Vertex::new(point))
    }
}
