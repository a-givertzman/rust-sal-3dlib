//!
//! Algorithms to create new objects from the combinations
//! of two groups of objects in 3-dimentional space.
//
pub mod volume;
//
use sal_3dlib_core::ops::boolean;
pub use sal_3dlib_core::ops::boolean::*;
///
/// Set of options, which allow speeding-up [Fuse], [Intersect] and [Cut] operations
/// (and other based on these ones such as [volume::Volume] and [volume::AlgoMakerVolume])
/// and improving the quality of the result.
///
/// See [General Fuse Algorithm / Options] for more details.
///
/// [General Fuse Algorithm / Options]: https://dev.opencascade.org/doc/overview/html/specification__boolean_operations.html#specification__boolean_7
#[derive(Clone, Copy)]
pub struct OpConf {
    ///
    /// Turns on running the operation algorithm in parallel mode.
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
