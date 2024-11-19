//! Basic operations to read from and write to file.
//
use crate::{prelude, topology::shape};
use opencascade::primitives::{self, ShapeType};
use sal_3dlib::props::Attributes;
use std::path::Path;
///
/// Reader of various file format.
pub struct Reader {
    nodes: Vec<(String, primitives::Shape)>,
}
//
//
impl Reader {
    ///
    /// Reading STEP file.
    pub fn read_step(filename: impl AsRef<Path>) -> Result<Self, primitives::ReadSTEPError> {
        primitives::read_step(filename).map(|nodes| Self { nodes })
    }
    ///
    /// Extract read buffer into the vector.
    pub fn into_vec<T>(self) -> anyhow::Result<Vec<(String, prelude::Shape<Option<T>>)>> {
        let mut elmnts = Vec::with_capacity(self.nodes.len());
        for (key, s) in self.nodes {
            let shape = match s.shape_type() {
                ShapeType::Vertex => {
                    let cad_vertex = primitives::Vertex::try_from(&s)?;
                    let local_vertex = shape::Vertex(cad_vertex);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_vertex = prelude::Vertex::from((local_vertex, attrs));
                    prelude::Shape::Vertex(export_vertex)
                }
                ShapeType::Edge => {
                    let cad_edge = primitives::Edge::try_from(&s)?;
                    let local_edge = shape::Edge(cad_edge);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_edge = prelude::Edge::from((local_edge, attrs));
                    prelude::Shape::Edge(export_edge)
                }
                ShapeType::Wire => {
                    let cad_wire = primitives::Wire::try_from(&s)?;
                    let local_wire = shape::Wire(cad_wire);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_wire = prelude::Wire::from((local_wire, attrs));
                    prelude::Shape::Wire(export_wire)
                }
                ShapeType::Face => {
                    let cad_face = primitives::Face::try_from(&s)?;
                    let local_face = shape::Face(cad_face);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_face = prelude::Face::from((local_face, attrs));
                    prelude::Shape::Face(export_face)
                }
                ShapeType::Shell => {
                    let cad_shell = primitives::Shell::try_from(&s)?;
                    let local_shell = shape::Shell(cad_shell);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_shell = prelude::Shell::from((local_shell, attrs));
                    prelude::Shape::Shell(export_shell)
                }
                ShapeType::Solid => {
                    let cad_solid = primitives::Solid::try_from(&s)?;
                    let local_solid = shape::Solid(cad_solid);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_solid = prelude::Solid::from((local_solid, attrs));
                    prelude::Shape::Solid(export_solid)
                }
                ShapeType::Compound => {
                    let cad_compound = primitives::Compound::try_from(&s)?;
                    let local_compound = shape::Compound(cad_compound);
                    let attrs = Attributes::new(key.clone(), None);
                    let export_compound = prelude::Compound::from((local_compound, attrs));
                    prelude::Shape::Compound(export_compound)
                }
                ShapeType::Shape => anyhow::bail!("Expected a basic topology type, but got Shape"),
                ShapeType::CompoundSolid => anyhow::bail!("CompoundSolid is not supported yet"),
            };
            elmnts.push((key, shape));
        }
        Ok(elmnts)
    }
}
