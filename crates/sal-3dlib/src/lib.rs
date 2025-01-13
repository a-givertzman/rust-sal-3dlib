//! Set of modules for interacting with the CAD kernel.
//!
//! Main objects and operations are defined in the core module (see [sal_3dlib_core]).
//!
//! [sal_occt_rs] provides an implementation of the core.
//
pub use sal_3dlib_core::bound;
pub use sal_occt_rs::fs;
pub use sal_occt_rs::gmath;
///
/// Operations for creating, transforming, and modifying objects.
pub mod ops {
    ///
    /// Create new objects from combinations of two groups of objects.
    pub mod boolean {
        ///
        /// Build an elementary volume from a set of objects.
        pub mod volume {
            pub use sal_3dlib_core::ops::boolean::volume::*;
            pub use sal_occt_rs::ops::boolean::volume::VolumeConf;
        }
        pub use sal_3dlib_core::ops::boolean::*;
        pub use sal_occt_rs::ops::boolean::OpConf;
    }
    pub use sal_3dlib_core::ops::{transform, Polygon, Solidify};
}
pub use sal_3dlib_core::props;
pub use sal_occt_rs::export::topology;
