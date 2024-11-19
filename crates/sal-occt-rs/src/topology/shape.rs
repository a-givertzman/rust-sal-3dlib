#![allow(dead_code)]
//
mod compound;
mod face;
mod shell;
mod solid;
mod vertex;
mod wire;
//
use opencascade::primitives;
///
/// Implementation of [sal_3dlib::topology::Vertex].
pub struct Vertex(pub(crate) primitives::Vertex);
///
/// Implementation of [sal_3dlib::topology::Edge].
pub struct Edge(pub(crate) primitives::Edge);
///
/// Implementation of [sal_3dlib::topology::Wire].
pub struct Wire(pub(crate) primitives::Wire);
///
/// Implementation of [sal_3dlib::topology::Face].
#[derive(Clone)]
pub struct Face(pub(crate) primitives::Face);
///
/// Implementation of [sal_3dlib::topology::Shell].
#[derive(Clone)]
pub struct Shell(pub(crate) primitives::Shell);
///
/// Implementation of [sal_3dlib::topology::Solid].
pub struct Solid(pub(crate) primitives::Solid);
///
/// Implementation of [sal_3dlib::topology::Compound].
pub struct Compound(pub(crate) primitives::Compound);
