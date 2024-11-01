use super::*;
use indexmap::map::IntoValues;
//
//
impl<const N: usize, V, E, W, F, L, D, T> Compound<N, V, E, W, F, L, D, T> {
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        shape: Shape<N, V, E, W, F, L, D, T>,
    ) -> Option<Shape<N, V, E, W, F, L, D, T>> {
        self.shapes.insert(key.into(), shape)
    }
    //
    //
    pub fn remove(&mut self, key: &str) -> Option<Shape<N, V, E, W, F, L, D, T>> {
        self.shapes.swap_remove(key)
    }
}
//
//
impl<const N: usize, V, E, W, F, L, D, T> IntoIterator for Compound<N, V, E, W, F, L, D, T> {
    type Item = Shape<N, V, E, W, F, L, D, T>;
    type IntoIter = IntoValues<String, Self::Item>;
    //
    //
    fn into_iter(self) -> Self::IntoIter {
        self.shapes.into_values()
    }
}
//
//
impl<const N: usize, T, V, E, W, F, L, D> Default for Compound<N, V, E, W, F, L, D, T> {
    fn default() -> Self {
        Self {
            shapes: Default::default(),
            attrs: Default::default(),
        }
    }
}
