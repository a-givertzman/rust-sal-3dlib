//!
//! Edge implementation in terms of OCCT.
//!
//! It provides the final object - [Edge] - and its related trait implementations.
//
use opencascade::primitives;
use sal_3dlib_core::topology::shape::edge;
///
/// Shape corresponding to a curve, and bound by a vertex at each extremity.
///
/// For internal use only. It provides low-level implementation for [Edge].
#[allow(dead_code)]
#[derive(Clone)]
pub struct OcctEdge(pub(crate) primitives::Edge);
///
/// Shape corresponding to a curve, and bound by a vertex at each extremity.
pub type Edge<T> = edge::Edge<3, OcctEdge, T>;
