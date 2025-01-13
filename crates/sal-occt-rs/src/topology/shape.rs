//! Inner structures used for [sal_3dlib_core::topology].
//
#![allow(dead_code)]
//
pub mod compound;
mod face;
mod shell;
mod solid;
mod vertex;
mod wire;
//
use opencascade::primitives;
///
/// Implementation of [sal_3dlib_core::topology::Vertex].
#[derive(Clone)]
pub struct Vertex(pub(crate) primitives::Vertex);
///
/// Implementation of [sal_3dlib_core::topology::Edge].
#[derive(Clone)]
pub struct Edge(pub(crate) primitives::Edge);
///
/// Implementation of [sal_3dlib_core::topology::Wire].
#[derive(Clone)]
pub struct Wire(pub(crate) primitives::Wire);
///
/// Implementation of [sal_3dlib_core::topology::Face].
#[derive(Clone)]
pub struct Face(pub(crate) primitives::Face);
///
/// Implementation of [sal_3dlib_core::topology::Shell].
#[derive(Clone)]
pub struct Shell(pub(crate) primitives::Shell);
///
/// Implementation of [sal_3dlib_core::topology::Solid].
#[derive(Clone)]
pub struct Solid(pub(crate) primitives::Solid);
///
/// Implementation of [sal_3dlib_core::topology::Compound].
#[derive(Clone)]
pub struct Compound(pub(crate) primitives::Compound);
