//! Implementation of [sal_3dlib_core::ops::boolean::volume].
//
use super::*;
use sal_3dlib_core::ops::boolean::volume;
///
/// See [sal_3dlib_core::ops::boolean::volume::VolumeConf] for details.
#[derive(Clone, Copy)]
pub struct VolumeConf {
    ///
    /// See [sal_3dlib_core::ops::boolean::OpConf::parallel] for details.
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
