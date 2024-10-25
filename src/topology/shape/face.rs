use super::*;
use crate::{
    bound::{Bound, BoundingBox},
    props::{Area, Center},
};
//
//
impl<const N: usize, F, V> Center for Face<N, F>
where
    F: Center<Output = V>,
{
    type Output = Vertex<N, V>;
    //
    //
    fn center(&self) -> Self::Output {
        Vertex {
            inner: self.inner.center(),
            attrs: None,
        }
    }
}

impl<const N: usize, T: Area> Area for Face<N, T> {
    fn area(&self) -> f64 {
        self.inner.area()
    }
}

impl<const N: usize, V, F> Bound<Vertex<N, V>> for Face<N, F>
where
    V: Clone,
    F: Bound<V>,
{
    fn bounding_box(&self) -> BoundingBox<Vertex<N, V>> {
        let bound = self.inner.bounding_box();
        BoundingBox::new_unchecked(
            Vertex::from(bound.min().clone()),
            Vertex::from(bound.max().clone()),
        )
    }
}
