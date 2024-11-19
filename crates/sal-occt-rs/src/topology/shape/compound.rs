use super::*;
use sal_3dlib::props::Center;
//
//
impl Center for Compound {
    type Output = Vertex;
    //
    //
    fn center(&self) -> Self::Output {
        let point = self.0.center_of_mass();
        Vertex(primitives::Vertex::new(point))
    }
}
