//!
//! Face implementation in terms of OCCT.
//!
//! It provides the final object - [Face] - and its related trait implementations.
//
use super::{
    vertex::{OcctVertex, Vertex},
    wire::OcctWire,
};
use crate::gmath::vector::Vector;
use glam::DVec3;
use opencascade::primitives;
use sal_3dlib_core::{
    ops::transform,
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
impl transform::Rotate<OcctVertex, Vector> for OcctFace {
    fn rotate(self, origin: OcctVertex, axis: Vector, rad: f64) -> Self {
        Self(self.0.rotated(origin.0, DVec3::from_array(*axis.0), rad))
    }
}
//
//
impl transform::Translate<Vector> for OcctFace {
    fn translate(self, dir: Vector) -> Self {
        Self(self.0.translated(DVec3::from_array(*dir.0)))
    }
}
///
/// Part of a surface bounded by a closed wire.
pub type Face<T> = face::Face<3, OcctFace, T>;
///
/// Defines a non-persistent rotation in 3-dimensional space.
///
/// This implementation is based on [the transformation algorithm],
/// and uses the part, which is related to rotation in space.
///
/// [the transformation algorithm]: https://dev.opencascade.org/doc/overview/html/occt_user_guides__modeling_algos.html#occt_modalg_3b
pub trait Rotate<T>: transform::Rotate<Vertex<T>, Vector> {
    ///
    /// Consumes `self` and returns a new rotated instance, where
    /// * `origin` - rotation pivot,
    /// * `axis` - axis to rotate around,
    /// * `rad` - angle in radians.
    ///
    /// # Examples
    /// Rotate a square 90 degrees around oX
    /// taking its center as the pivot point:
    /// ```no_run
    /// # mod sal_3dlib {
    /// #     pub use sal_occt_rs::*;
    /// #     pub use sal_3dlib_core::props;
    /// # };
    /// use core::f64::consts::FRAC_PI_2;
    /// use sal_3dlib::{
    ///     gmath::{point::Point, vector::Vector},
    ///     props::Center,
    ///     topology::shape::{
    ///         face::{Face, Rotate},
    ///         vertex::Vertex,
    ///         wire::{Polygon, Wire},
    ///     },
    /// };
    /// //
    /// // initialize corners
    /// let a = Vertex::new(*Point::from([0.0, 0.0, 0.0]));
    /// let b = Vertex::new(*Point::from([0.0, 1.0, 0.0]));
    /// let c = Vertex::new(*Point::from([1.0, 1.0, 0.0]));
    /// let d = Vertex::new(*Point::from([1.0, 0.0, 0.0]));
    /// // create a closed rectangle by connecting the corners
    /// let rect = Wire::polygon([a, b, c, d], true)?;
    /// // fill the space between edges of the rectangle
    /// let mut square = Face::<()>::try_from(&rect)?;
    /// // rotate it around oX
    /// let center = square.center();
    /// square = square.rotate(center, Vector::unit_x(), FRAC_PI_2);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    fn rotate(self, origin: Vertex<T>, axis: Vector, rad: f64) -> Self
    where
        Self: Sized,
    {
        <Self as transform::Rotate<Vertex<T>, Vector>>::rotate(self, origin, axis, rad)
    }
}
///
/// Defines a non-persistent translation in 3-dimensional space.
///
/// This implementation is based on [the transformation algorithm],
/// and uses the part, which is related to moving in space.
///
/// [the transformation algorithm]: https://dev.opencascade.org/doc/overview/html/occt_user_guides__modeling_algos.html#occt_modalg_3b
pub trait Translate: transform::Translate<Vector> {
    ///
    /// Consumes `self` and returns a new translated instance moved to `dir`.
    ///
    /// # Examples
    /// Move a square placed on oXY 1 unit up (along oZ):
    /// ```no_run
    /// # mod sal_3dlib {
    /// #     pub use sal_occt_rs::*;
    /// # };
    /// # //
    /// use sal_3dlib::{
    ///     gmath::{point::Point, vector::Vector},
    ///     topology::shape::{
    ///         face::{Face, Translate},
    ///         vertex::Vertex,
    ///         wire::{Polygon, Wire},
    ///     },
    /// };
    /// //
    /// // initialize corners
    /// let a = Vertex::new(*Point::from([0.0, 0.0, 0.0]));
    /// let b = Vertex::new(*Point::from([0.0, 1.0, 0.0]));
    /// let c = Vertex::new(*Point::from([1.0, 1.0, 0.0]));
    /// let d = Vertex::new(*Point::from([1.0, 0.0, 0.0]));
    /// // create a closed rectangle by connecting the corners
    /// let rect = Wire::polygon([a, b, c, d], true)?;
    /// // fill the space between edges of the rectangle
    /// let mut square = Face::<()>::try_from(&rect)?;
    /// // move it up 1 unit along oZ
    /// square = square.translate(Vector::unit_z());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    fn translate(self, dir: Vector) -> Self
    where
        Self: Sized,
    {
        <Self as transform::Translate<Vector>>::translate(self, dir)
    }
}
//
//
impl<T> Rotate<T> for Face<T> {}
impl<T> Translate for Face<T> {}
