use super::*;
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
