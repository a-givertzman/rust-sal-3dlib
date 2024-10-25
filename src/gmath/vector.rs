use super::*;
//
//
impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(value: [f64; N]) -> Self {
        Self(value)
    }
}
