use super::*;
//
//
impl<const N: usize> std::ops::Deref for Point<N> {
    type Target = [f64; N];
    //
    //
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
