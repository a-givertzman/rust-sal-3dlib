//! Implementation of [sal_3dlib::ops::boolean].
//
pub mod volume;
//
use sal_3dlib::ops::boolean;
///
/// See [sal_3dlib::ops::boolean::OpConf] for details.
#[derive(Clone, Copy)]
pub struct OpConf {
    ///
    /// See [sal_3dlib::ops::boolean::OpConf::parallel] for details.
    pub parallel: bool,
}
//
//
impl From<OpConf> for boolean::OpConf {
    fn from(value: OpConf) -> Self {
        Self {
            parallel: value.parallel,
        }
    }
}