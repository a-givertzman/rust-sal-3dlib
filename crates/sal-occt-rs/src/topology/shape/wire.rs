use super::*;
use crate::gmath::Point;
use glam::DVec3;
use sal_3dlib::ops::Polygon;
//
//
impl Polygon<Vertex> for Wire {
    type Error = opencascade::Error;
    ///
    /// Creates a polygon from ordered points.
    fn polygon(vxs: impl IntoIterator<Item = Vertex>, _: bool) -> Result<Self, Self::Error> {
        let points = vxs.into_iter().map(|v| {
            let point = Point::from(&v);
            DVec3::from_array(*point)
        });
        primitives::Wire::from_ordered_points(points).map(Self)
    }
}
