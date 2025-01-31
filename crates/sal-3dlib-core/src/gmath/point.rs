use super::Point;
use std::ops::Deref;
//
//
impl<const N: usize> Deref for Point<N> {
    type Target = [f64; N];
    //
    //
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//
//
impl<const N: usize> From<[f64; N]> for Point<N> {
    fn from(value: [f64; N]) -> Self {
        Self(value)
    }
}
