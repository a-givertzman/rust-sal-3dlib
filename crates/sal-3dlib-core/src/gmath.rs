//!
//! Mathematical structures in N-dimensional space, which are independent of any CAD kernel.
//
mod point;
mod vector;
///
/// Location in N-dimensional space.
#[derive(Clone, Copy)]
pub struct Point<const N: usize>(pub(crate) [f64; N]);
///
/// Displacment in N-dimensional space.
#[derive(Clone, Copy)]
pub struct Vector<const N: usize>(pub(crate) [f64; N]);
