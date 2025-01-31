use crate::topology::shape::vertex::OcctVertex;
use sal_3dlib_core::gmath;
///
/// Location in 3-dimensional space.
///
/// # Examples
/// ```no_run
/// # mod sal_3dlib {
/// #     pub use sal_occt_rs::*;
/// # };
/// # //
/// use sal_3dlib::{
///     gmath::point::Point,
///     topology::shape::vertex::Vertex,
/// };
/// //
/// // create the origin point
/// let o = Point::from([0.0, 0.0, 0.0]);
/// // Cast the point to Vertex and then back.
/// // Note that Point can be dereferenced
/// // to its natural representation of type [f64; 3].
/// let o_v = Vertex::<()>::new(*o);
/// // check whether `o_v` points to the origin
/// assert_eq!(*o, o_v.point());
/// ```
pub type Point = gmath::Point<3>;
//
//
impl From<&OcctVertex> for Point {
    fn from(vertex: &OcctVertex) -> Self {
        let v = &vertex.0;
        Self::from([v.x(), v.y(), v.z()])
    }
}
