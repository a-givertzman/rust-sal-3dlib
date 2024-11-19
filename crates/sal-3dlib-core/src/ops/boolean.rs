//! Create new objects from combinations of two groups of objects.
//
pub mod volume;
///
/// Union of two groups.
pub trait Fuse<Rhs, C, O>
where
    OpConf: From<C>,
{
    /// Returns the union of `&self` and `&rhs`. It's configured using `conf`.
    fn fuse(&self, rhs: &Rhs, conf: C) -> O;
}
///
/// Intersection of two groups.
pub trait Intersect<Rhs, C, O>
where
    OpConf: From<C>,
{
    /// Returns the common part of `&self` and `&rhs`. It's configured using `conf`.
    fn intersect(&self, rhs: &Rhs, conf: C) -> O;
}
///
/// Difference between two groups.
pub trait Cut<Rhs, C, O>
where
    OpConf: From<C>,
{
    /// Returns the difference between `&self` and `&rhs`. It's configured using `conf`.
    fn cut(&self, rhs: &Rhs, conf: C) -> O;
}
///
/// Config to perform [Fuse], [Intersect] and [Cut].
pub struct OpConf {
    ///
    /// Whether parallel processing is enabled.
    pub parallel: bool,
}
