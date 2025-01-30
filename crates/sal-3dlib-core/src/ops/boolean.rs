//!
//! Algorithms to create new objects from combinations of two groups of objects.
//
pub mod volume;
///
/// Union of two groups.
pub trait Fuse<Rhs, Conf, Output>
where
    OpConf: From<Conf>,
{
    ///
    /// Returns the union of `&self` and `&rhs`.
    ///
    /// Use `conf` to configure the operation.
    fn fuse(&self, rhs: &Rhs, conf: Conf) -> Output;
}
///
/// Intersection of two groups.
pub trait Intersect<Rhs, Conf, Output>
where
    OpConf: From<Conf>,
{
    ///
    /// Returns the common part of `&self` and `&rhs`.
    ///
    /// Use `conf` to configure the operation.
    fn intersect(&self, rhs: &Rhs, conf: Conf) -> Output;
}
///
/// Difference between two groups.
pub trait Cut<Rhs, Conf, Output>
where
    OpConf: From<Conf>,
{
    ///
    /// Returns the difference between `&self` and `&rhs`.
    ///
    /// Use `conf` to configure the operation.
    fn cut(&self, rhs: &Rhs, conf: Conf) -> Output;
}
///
/// Set of options, which allow speeding-up [Fuse], [Intersect] and [Cut] operations
///
/// Note that [OpConf] is not considered to be used directly.
/// Each kernel implementation should provide its specified version (wrapper),
/// which can be cast (e. g. via implementing [From]) to the current one.
pub struct OpConf {
    pub parallel: bool,
}
