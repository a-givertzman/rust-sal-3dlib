//! Build an elementary volume from a set of objects.
//
use super::*;
///
/// Volume creation algorithm.
pub trait Volume<Rhs, C, O>
where
    VolumeConf: From<C>,
{
    ///
    /// Returns the volume of `&self` and `&rhs`. It's configured using `conf`.
    fn volume(&self, rhs: &Rhs, conf: C) -> O;
}
///
/// Configuration used to perform [Volume].
pub struct VolumeConf {
    ///
    /// [Volume] is based on [crate::ops::boolean] operations,
    /// so all fields of [OpConf] also make sense.
    pub op_conf: OpConf,
}
///
/// Algorithm to volume together three sets of the objects.
pub trait AlgoMakerVolume<A, B, C> {
    type Error;
    //
    //
    fn build<'a>(
        a: impl IntoIterator<Item = &'a A>,
        b: impl IntoIterator<Item = &'a B>,
        c: impl IntoIterator<Item = &'a C>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        A: 'a,
        B: 'a,
        C: 'a;
}
