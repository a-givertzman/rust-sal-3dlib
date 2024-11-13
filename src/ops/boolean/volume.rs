//! Build an elementary volume from a set of objects.
//
use super::*;
///
/// Volume creation algorithm.
pub trait Volume<Rhs, C>
where
    VolumeConf: From<C>,
{
    type Output;
    ///
    /// Returns the volume of `&self` and `&rhs`. It's configured using `conf`.
    fn volume(&self, rhs: &Rhs, conf: C) -> Self::Output;
}
///
/// Configuration used to perform [Volume].
pub struct VolumeConf {
    ///
    /// [Volume] is based on [crate::ops::boolean] operations,
    /// so all fields of [OpConf] also make sense.
    pub op_conf: OpConf,
}
