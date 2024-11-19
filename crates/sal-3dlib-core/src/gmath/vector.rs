use super::*;
use std::ops::Deref;
//
//
impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(value: [f64; N]) -> Self {
        Self(value)
    }
}
//
//
impl<const N: usize> Deref for Vector<N> {
    type Target = [f64; N];
    //
    //
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
