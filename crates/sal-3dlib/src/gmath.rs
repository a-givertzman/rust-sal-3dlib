//! Mathematical structures that are independent of any CAD.
//
mod point;
mod vector;
///
/// Location in N-dimensional space.
pub struct Point<const N: usize>(pub(crate) [f64; N]);
///
/// Displacment in N-dimensional space.
pub struct Vector<const N: usize>(pub(crate) [f64; N]);
