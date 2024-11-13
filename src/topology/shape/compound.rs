use super::*;
//
//
impl<const N: usize, C, T> From<(C, Attributes<T>)> for Compound<N, C, T> {
    fn from((compound, attrs): (C, Attributes<T>)) -> Self {
        Self {
            inner: compound,
            attrs: Some(attrs),
        }
    }
}
