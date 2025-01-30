//!
//! Build an elementary volume from a set of objects.
//
use super::OpConf;
use sal_3dlib_core::ops::boolean::volume;
pub use sal_3dlib_core::ops::boolean::volume::*;
///
/// Set of options, which allow speeding-up [Volume] operation and [AlgoMakerVolume] algorithm.
///
/// This configuration based on the configuration of [General Fuse (GF) algorithm],
/// thus all the options of the GF algorithm (see [super::OpConf]) are also available.
///
/// [General Fuse (GF) algorithm]: https://dev.opencascade.org/doc/overview/html/specification__boolean_operations.html#specification__boolean_7
#[derive(Clone, Copy)]
pub struct VolumeConf {
    ///
    /// Turns on running the \[operation\] algorithm in parallel mode.
    pub parallel: bool,
}
//
//
impl From<VolumeConf> for volume::VolumeConf {
    fn from(value: VolumeConf) -> Self {
        Self {
            op_conf: OpConf {
                parallel: value.parallel,
            }
            .into(),
        }
    }
}
