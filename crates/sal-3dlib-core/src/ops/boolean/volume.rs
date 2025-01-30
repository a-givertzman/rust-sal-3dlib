//!
//! Build an elementary volume from a set of objects.
//
use super::*;
///
/// Volume creation algorithm.
pub trait Volume<Rhs, Conf, Output>
where
    VolumeConf: From<Conf>,
{
    ///
    /// Returns the volume of `&self` and `&rhs`.
    ///
    /// Use `conf` to configure the operation.
    fn volume(&self, rhs: &Rhs, conf: Conf) -> Output;
}
///
/// Set of options, which allow speeding-up [Volume] operation and [AlgoMakerVolume] algorithm.
pub struct VolumeConf {
    ///
    /// Set of options, which allow speeding-up [Fuse], [Intersect] and [Cut] operations
    ///
    /// This configuration based on the configuration of [super::OpConf],
    /// thus all its options are inherited and also available.
    pub op_conf: OpConf,
}
///
/// Algorithm to volume together three sets of the objects.
pub trait AlgoMakerVolume<T1, T2, T3> {
    //
    //
    type Error;
    //
    // Performs the algorithm and returns its output result.
    fn build<'a>(
        set1: impl IntoIterator<Item = &'a T1>,
        set2: impl IntoIterator<Item = &'a T2>,
        set3: impl IntoIterator<Item = &'a T3>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T1: 'a,
        T2: 'a,
        T3: 'a;
}
