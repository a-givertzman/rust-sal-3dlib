//!
//! Basic operations to read from and write to files.
//
use crate::topology::shape::{
    compound::{Compound, OcctCompound},
    edge::{Edge, OcctEdge},
    face::{Face, OcctFace},
    shell::{OcctShell, Shell},
    solid::{OcctSolid, Solid},
    vertex::{OcctVertex, Vertex},
    wire::{OcctWire, Wire},
    Shape,
};
use opencascade::primitives::{self, ShapeType};
use sal_3dlib_core::props::Attributes;
use std::path::Path;
///
/// Reader of various file formats.
pub struct Reader {
    nodes: Vec<(String, primitives::Shape)>,
}
//
//
impl Reader {
    ///
    /// Reads the STEP file.
    pub fn read_step(filename: impl AsRef<Path>) -> Result<Self, primitives::ReadSTEPError> {
        primitives::read_step(filename).map(|nodes| Self { nodes })
    }
    ///
    /// Extracts read buffer into the vector.
    pub fn into_vec<T>(self, build_attributes: impl Fn(String, primitives::Shape) -> T + 'static) -> anyhow::Result<Vec<(String, Shape<T>)>> {
        let mut elmnts = Vec::with_capacity(self.nodes.len());
        for (key, shape) in self.nodes {
            let shape = match shape.shape_type() {
                ShapeType::Vertex => {
                    let cad_vertex = primitives::Vertex::try_from(&shape)?;
                    let occt_vertex = OcctVertex(cad_vertex);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let vertex = Vertex::from((occt_vertex, attrs));
                    Shape::Vertex(vertex)
                }
                ShapeType::Edge => {
                    let cad_edge = primitives::Edge::try_from(&shape)?;
                    let occt_edge = OcctEdge(cad_edge);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let edge = Edge::from((occt_edge, attrs));
                    Shape::Edge(edge)
                }
                ShapeType::Wire => {
                    let cad_wire = primitives::Wire::try_from(&shape)?;
                    let occt_wire = OcctWire(cad_wire);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let wire = Wire::from((occt_wire, attrs));
                    Shape::Wire(wire)
                }
                ShapeType::Face => {
                    let cad_face = primitives::Face::try_from(&shape)?;
                    let occt_face = OcctFace(cad_face);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let face = Face::from((occt_face, attrs));
                    Shape::Face(face)
                }
                ShapeType::Shell => {
                    let cad_shell = primitives::Shell::try_from(&shape)?;
                    let occt_shell = OcctShell(cad_shell);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let shell = Shell::from((occt_shell, attrs));
                    Shape::Shell(shell)
                }
                ShapeType::Solid => {
                    let cad_solid = primitives::Solid::try_from(&shape)?;
                    let occt_solid = OcctSolid(cad_solid);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let solid = Solid::from((occt_solid, attrs));
                    Shape::Solid(solid)
                }
                ShapeType::Compound => {
                    let cad_compound = primitives::Compound::try_from(&shape)?;
                    let occt_compound = OcctCompound(cad_compound);
                    let attrs = Attributes::new(
                        key.clone(),
                        build_attributes(key.clone(), shape),
                    );
                    let compound = Compound::from((occt_compound, attrs));
                    Shape::Compound(compound)
                }
                ShapeType::Shape => anyhow::bail!("Expected a basic topology type, but got Shape"),
                ShapeType::CompoundSolid => anyhow::bail!("CompoundSolid is not supported yet"),
            };
            elmnts.push((key, shape));
        }
        Ok(elmnts)
    }
}
