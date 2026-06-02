use parry3d_f64::math::Vec3;
use parry3d_f64::shape::{TriMesh, TriMeshFlags};
use sal_core::error::Error;
use std::io::Write;
use std::path::*;

///
/// Load data from .stl file
pub fn load(path: &Path, model_scale: f64) -> Result<TriMesh, Error> {
    let error = Error::new("Shape", "load_stl");
    let scale = if model_scale > 0. && model_scale != 1. { Some(1. / model_scale) } else { None };
    
    let file = std::fs::File::open(path)
        .map_err(|err| error.pass_with(format!("File::open, path:{:?}", path), err.to_string()))?;
    
    let mut reader = std::io::BufReader::new(file);
    
    let stl_mesh = stl_io::read_stl(&mut reader)
        .map_err(|err| error.pass_with("stl_io::read_stl", err.to_string()))?;

    // 1. Выгружаем вершины
    let vertices = stl_mesh
        .vertices
        .iter()
        .map(|v| Vec3::new(v[0] as f64, v[1] as f64, v[2] as f64))
        .collect::<Vec<_>>();

    // 2. Формируем массив индексов
    let indices = stl_mesh
        .faces
        .into_iter()
        .map(|f| {
            [
                f.vertices[0] as u32,
                f.vertices[1] as u32,
                f.vertices[2] as u32,
            ]
        })
        .collect::<Vec<_>>();

    // созданиe меша и последующеe масштабированиe
    let mesh = TriMesh::with_flags(vertices, indices, TriMeshFlags::all())
        .map_err(|err| error.pass_with("TriMesh::with_flags", err.to_string()))
        .map(|m| {
            if let Some(scale) = scale { 
                m.scaled(Vec3::splat(scale))
            } else {
                m
            }
        });

    mesh
}

/// Запись данных TriMesh в .stl файл
pub fn write(path: &Path, mesh: &TriMesh) -> Result<(), Error> {
    let error = Error::new("Shape", "write_stl");
    
    let (result, empty_normals): (Vec<_>, Vec<_>) = mesh
        .triangles()
        .map(|t| (t.normal(), t))
        .partition(|(n, _)| n.is_some());
        
    if !empty_normals.is_empty() {
        return Err(error.err(format!("calculate normal error, path:{:?}", path)));
    }
    
    let triangles: Vec<_> = result
        .into_iter()
        .map(|(n, t)| {
            let n = n.unwrap();
            // В parry3d_f64 нормали и точки возвращаются как f64 (Vec3), приводим к f32 для STL
            let normal = stl_io::Vector([n.x as f32, n.y as f32, n.z as f32]);
            let vertices = [
                stl_io::Vector([t.a.x as f32, t.a.y as f32, t.a.z as f32]),
                stl_io::Vector([t.b.x as f32, t.b.y as f32, t.b.z as f32]),
                stl_io::Vector([t.c.x as f32, t.c.y as f32, t.c.z as f32]),
            ];
            stl_io::Triangle { normal, vertices }
        })
        .collect();
        
    let mut binary_stl = Vec::<u8>::new();
    stl_io::write_stl(&mut binary_stl, triangles.iter())
        .map_err(|err| error.pass_with("stl_io::write_stl", err.to_string()))?;
        
    let mut buffer = std::fs::File::create(path)
        .map_err(|err| error.pass_with(format!("File::create, path:{:?}", path), err.to_string()))?;
        
    buffer.write_all(&binary_stl)
        .map_err(|err| error.pass_with(format!("buffer.write_all, path:{:?}", path), err.to_string()))
}