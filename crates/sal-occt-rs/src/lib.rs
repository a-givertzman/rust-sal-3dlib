//!
//! Implementation of [sal_3dlib_core] taking [Open CASCADE Technology] library as the CAD kernel.
//!
//! This is implemented with use of high-level binding provided by [opencascade].
//!
//! [Open CASCADE Technology]: https://dev.opencascade.org/doc/overview/html/index.html
//
pub mod fs;
pub mod gmath;
pub mod ops;
#[cfg(test)]
mod tests;
pub mod topology;
