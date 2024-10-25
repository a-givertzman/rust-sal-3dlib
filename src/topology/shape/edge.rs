use super::*;
use crate::gmath::Point;
//
//
impl<const N: usize, T> Edge<N, T>
where
    T: From<[Point<N>; 2]>,
    Point<N>: for<'a> From<&'a T>,
{
    pub fn new(front: impl Into<Point<N>>, back: impl Into<Point<N>>) -> Self {
        Self {
            inner: T::from([front.into(), back.into()]),
            attrs: None,
        }
    }
}
