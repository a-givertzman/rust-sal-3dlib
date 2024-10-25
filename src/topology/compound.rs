use super::*;
use indexmap::map::IntoValues;
//
//
impl<const N: usize, V, E, W, F, L, D> Compound<N, V, E, W, F, L, D> {
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        shape: Shape<N, V, E, W, F, L, D>,
    ) -> Option<Shape<N, V, E, W, F, L, D>> {
        self.shapes.insert(key.into(), shape)
    }
    //
    //
    pub fn remove(&mut self, key: &str) -> Option<Shape<N, V, E, W, F, L, D>> {
        self.shapes.swap_remove(key)
    }
}
//
//
impl<const N: usize, V, E, W, F, L, D> IntoIterator for Compound<N, V, E, W, F, L, D> {
    type Item = Shape<N, V, E, W, F, L, D>;
    type IntoIter = IntoValues<String, Self::Item>;
    //
    //
    fn into_iter(self) -> Self::IntoIter {
        self.shapes.into_values()
    }
}
//
//
impl<const N: usize, V, E, W, F, L, D> Default for Compound<N, V, E, W, F, L, D> {
    fn default() -> Self {
        Self {
            shapes: Default::default(),
            attrs: Default::default(),
        }
    }
}
