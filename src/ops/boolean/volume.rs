use super::*;
//
//
pub trait Volume<Rhs, C>
where
    VolumeConf: From<C>,
{
    type Output;
    //
    //
    fn volume(&self, rhs: Rhs, conf: C) -> Self::Output;
}
///
/// Provides options for [Volume] operation:
///  - `op_conf` - general options
///  - `...` - operation specific options
pub struct VolumeConf {
    ///
    /// [Volume] is based on [crate::ops] general operations,
    /// therefore all the options of [OpConf] are also available
    pub op_conf: OpConf,
    ///
    /// Set to true to exclude from the result any shapes internal to the solids
    pub avoid_internal_shapes: Option<bool>,
}
