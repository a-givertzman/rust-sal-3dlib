//! Implementation of [sal_3dlib_core] taking [opencascade] as the CAD kernel.
//
pub mod fs;
pub mod gmath;
pub mod ops;
#[cfg(test)]
mod tests;
mod topology;
pub mod export {
    ///
    /// Implementation of [sal_3dlib_core::topology].
    pub mod topology {
        ///
        /// Implementation of [sal_3dlib_core::topology].
        pub mod shape {
            use sal_3dlib_core::topology;
            ///
            /// See [sal_3dlib_core::topology::Vertex] for details.
            pub type Vertex<T> = topology::Vertex<3, super::Vertex, T>;
            ///
            /// See [sal_3dlib_core::topology::Edge] for details.
            pub type Edge<T> = topology::Edge<3, super::Edge, T>;
            ///
            /// See [sal_3dlib_core::topology::Wire] for details.
            pub type Wire<T> = topology::Wire<3, super::Wire, T>;
            ///
            /// See [sal_3dlib_core::topology::Face] for details.
            pub type Face<T> = topology::Face<3, super::Face, T>;
            ///
            /// See [sal_3dlib_core::topology::Shell] for details.
            pub type Shell<T> = topology::Shell<3, super::Shell, T>;
            ///
            /// See [sal_3dlib_core::topology::Solid] for details.
            pub type Solid<T> = topology::Solid<3, super::Solid, T>;
            ///
            /// See [sal_3dlib_core::topology::Compound] for details.
            pub type Compound<T> = topology::Compound<3, super::Compound, T>;
        }
        //
        use crate::topology::shape::*;
        use sal_3dlib_core::topology;
        ///
        /// See [sal_3dlib_core::topology::Shape] for details.
        pub type Shape<T> = topology::Shape<3, Vertex, Edge, Wire, Face, Shell, Solid, Compound, T>;
    }
}
///
/// Re-export of triats and structures (all in one) for easy access.
pub mod prelude {
    use super::*;
    pub use export::topology::{shape::*, *};
    pub use gmath::*;
    pub use ops::boolean::{volume::*, *};
}
