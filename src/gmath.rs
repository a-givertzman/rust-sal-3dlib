mod point;
mod vector;
//
//
pub struct Point<const N: usize>(pub(crate) [f64; N]);
//
//
pub struct Vector<const N: usize>(pub(crate) [f64; N]);
