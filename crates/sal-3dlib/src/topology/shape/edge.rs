use super::*;
//
//
impl<const N: usize, E, T> From<(E, Attributes<T>)> for Edge<N, E, T> {
    fn from((edge, attrs): (E, Attributes<T>)) -> Self {
        Self {
            inner: edge,
            attrs: Some(attrs),
        }
    }
}
