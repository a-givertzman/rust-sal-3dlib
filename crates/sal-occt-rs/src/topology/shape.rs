//!
//! Shape implementation in terms of [OCCT].
//!
//! It provides the final object - [Shape] - and its related trait implementations.
//!
//! [OCCT]: https://dev.opencascade.org/doc/overview/html/occt_user_guides__modeling_data.html#occt_modat_5
//
pub mod compound;
pub mod edge;
pub mod face;
pub mod shell;
pub mod solid;
pub mod vertex;
pub mod wire;
//
use sal_3dlib_core::topology::shape;
///
/// Abstract topological data structure describes a basic entity.
pub type Shape<T> = shape::Shape<
    3,
    vertex::OcctVertex,
    edge::OcctEdge,
    wire::OcctWire,
    face::OcctFace,
    shell::OcctShell,
    solid::OcctSolid,
    compound::OcctCompound,
    T,
>;
