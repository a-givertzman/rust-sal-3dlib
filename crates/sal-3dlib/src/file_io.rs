use parry3d_f64::math::Vec3;
use parry3d_f64::shape::{TriMesh, TriMeshFlags};
use sal_core::error::Error;
use std::io::Write;
use std::path::*;


///
/// Load data from .obj file
pub fn load_stl(path: &Path, model_scale: f64) -> Result<TriMesh, Error> {
    let error = Error::new("Shape", "load_stl");
    let scale = if model_scale > 0. && model_scale != 1. { Some(1. / model_scale) } else { None };
    
    let file = std::fs::File::open(path)
        .map_err(|err| error.pass_with(format!("File::open, path:{:?}", path), err.to_string()))?;
    let mut reader = std::io::BufReader::new(file);
    
    let stl_mesh = stl_io::read_stl(&mut reader)
        .map_err(|err| error.pass_with("stl_io::read_stl", err.to_string()))?;

    // 1. Выгружаем вершины в формате f64 (копирование через .iter() сохраняет stl_mesh)
    let vertices = stl_mesh
        .vertices
        .iter()
        .map(|v| Vec3::new(v[0] as f64, v[1] as f64, v[2] as f64))
        .collect::<Vec<_>>();

    // 2. Формируем массив индексов с автоматической валидацией порядка обхода
    let indices = stl_mesh
        .faces
        .into_iter()
        .map(|f| {
            let i0 = f.vertices[0] as u32;
            let i1 = f.vertices[1] as u32;
            let i2 = f.vertices[2] as u32;

            let p0 = vertices[i0 as usize];
            let p1 = vertices[i1 as usize];
            let p2 = vertices[i2 as usize];

            // Математическая нормаль по правилу Counter-Clockwise (CCW)
            let geom_normal = (p1 - p0).cross(p2 - p0);

            // Оригинальная нормаль, которую записала CAD-система в STL файл
            let file_normal = Vec3::new(f.normal[0] as f64, f.normal[1] as f64, f.normal[2] as f64);

            // Если они смотрят в разные стороны (скалярное произведение < 0), 
            // значит треугольник в файле записан в порядке CW. Инвертируем его в CCW.
            if geom_normal.dot(file_normal) < 0.0 {
                [i0, i2, i1]
            } else {
                [i0, i1, i2]
            }
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
pub fn write_stl(path: &Path, mesh: &TriMesh) -> Result<(), Error> {
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