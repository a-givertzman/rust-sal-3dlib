//!
//! Wire implementation in terms of OCCT.
//!
//! It provides the final object - [Wire] - and its related trait implementations.
//
use super::vertex::{OcctVertex, Vertex};
use crate::gmath::point::Point;
use glam::DVec3;
use opencascade::primitives;
use sal_3dlib_core::{ops, topology::shape::wire};
///
/// Sequence of edges connected by their vertices.
///
/// For internal use only. It provides low-level implementation for [Wire].
#[derive(Clone)]
pub struct OcctWire(pub(crate) primitives::Wire);
//
//
impl ops::Polygon<OcctVertex> for OcctWire {
    type Error = opencascade::Error;
    //
    //
    fn polygon(vs: impl IntoIterator<Item = OcctVertex>, _: bool) -> Result<Self, Self::Error> {
        let points = vs.into_iter().map(|v| {
            let point = Point::from(&v);
            DVec3::from_array(*point)
        });
        primitives::Wire::from_ordered_points(points).map(Self)
    }
}
///
/// Sequence of edges connected by their vertices.
pub type Wire<T> = wire::Wire<3, OcctWire, T>;
///
/// Algorithm to build polygonal wires from vertices or points.
pub trait Polygon<T>: ops::Polygon<Vertex<T>> {
    ///
    /// Creates a polygon from ordered vertices.
    ///
    /// Set `closed` to _true_ to get the closed result polygon,
    /// i. e. its start and end are the same vertices.
    ///
    /// See [Modeling Algorithms / Polygon] for details.
    ///
    /// # Examples
    /// Create a sqaure plane from four vertices:
    /// ```no_run
    /// use sal_occt_rs::topology::shape::{
    ///     face::Face,
    ///     vertex::Vertex,
    ///     wire::{Polygon, Wire},
    /// };
    /// use sal_occt_rs::gmath::point::Point;
    /// //
    /// // initialize corners
    /// let a = Vertex::new(*Point::from([0.0, 0.0, 0.0]));
    /// let b = Vertex::new(*Point::from([0.0, 1.0, 0.0]));
    /// let c = Vertex::new(*Point::from([1.0, 1.0, 0.0]));
    /// let d = Vertex::new(*Point::from([1.0, 0.0, 0.0]));
    /// // create a closed rectangle by connecting the corners
    /// let rect = Wire::polygon([a, b, c, d], true)?;
    /// // fill the space between edges of the rectangle
    /// let square = Face::<()>::try_from(&rect)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [Modeling Algorithms / Polygon]: https://dev.opencascade.org/doc/overview/html/occt_user_guides__modeling_algos.html#occt_modalg_3_4
    fn polygon(
        vertices: impl IntoIterator<Item = Vertex<T>>,
        closed: bool,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        <Self as ops::Polygon<Vertex<T>>>::polygon(vertices, closed)
    }
}
//
//
impl<T> Polygon<T> for Wire<T> {}
