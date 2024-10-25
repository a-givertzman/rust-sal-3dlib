pub mod volume;
//
// aka union
pub trait Fuse<Rhs, C>
where
    OpConf: From<C>,
{
    type Output;
    //
    //
    fn fuse(&self, rhs: &Rhs, conf: C) -> Self::Output;
}
//
// aka common
pub trait Intersect<Rhs, C>
where
    OpConf: From<C>,
{
    type Output;
    //
    //
    fn intersect(&self, rhs: &Rhs, conf: C) -> Self::Output;
}
//
// aka difference
pub trait Cut<Rhs, C>
where
    OpConf: From<C>,
{
    type Output;
    //
    //
    fn cut(&self, rhs: &Rhs, conf: C) -> Self::Output;
}
///
/// Provides options for [Fuse], [Intersect] and [Cut] operations
pub struct OpConf {
    ///
    /// Set additional tolerance for the operation
    pub tolerance: f64,
}
