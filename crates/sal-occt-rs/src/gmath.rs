//! Implementation of [sal_3dlib_core::gmath].
//
mod point;
mod vector;
//
use sal_3dlib_core::gmath;
///
/// Location in 3-dimensional space.
///
/// See [sal_3dlib_core::gmath::Point] for details.
pub type Point = gmath::Point<3>;
///
/// Displacment in 3-dimensional space.
///
/// See [sal_3dlib_core::gmath::Vector] for details.
pub struct Vector(pub(crate) gmath::Vector<3>);
