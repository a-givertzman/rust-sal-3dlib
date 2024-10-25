pub trait Rotate<O, A> {
    fn rotate(self, origin: O, axis: A, rad: f64) -> Self;
}
//
//
pub trait Translate<T> {
    fn translate(self, dir: T) -> Self;
}
