use parry3d_f64::math::{Mat3, Vec3};
use parry3d_f64::shape::{TriMesh, TriMeshFlags};

use crate::Plane;

pub fn expand_closed_ship_trimesh(mesh: &TriMesh, offset: f64) -> TriMesh {
    let vertices = mesh.vertices();
    let indices = mesh.indices();

    let mut face_planes = Vec::with_capacity(indices.len());
    let mut vertex_to_faces = vec![Vec::new(); vertices.len()];

    // 1. Предрасчет плоскостей всех граней
    for (face_idx, &triangle) in indices.iter().enumerate() {
        let [idx0_u32, idx1_u32, idx2_u32] = triangle;
        let idx0 = idx0_u32 as usize;
        let idx1 = idx1_u32 as usize;
        let idx2 = idx2_u32 as usize;

        let p0 = vertices[idx0];
        let p1 = vertices[idx1];
        let p2 = vertices[idx2];

        let v01 = p1 - p0;
        let v02 = p2 - p0;
        
        let mut normal = v01.cross(v02);
        if normal.length_squared() > 1e-12 {
            normal = normal.normalize();
        } else {
            normal = Vec3::Z; // Защитный дефолт для вырожденных треугольников
        }

        // Вычисляем смещенное d плоскости (уравнение плоскости: n · x = d)
        let shifted_d = normal.dot(p0) + offset;
        face_planes.push(Plane { normal, d: shifted_d });

        // Строим карту смежности вершин и граней
        vertex_to_faces[idx0].push(face_idx);
        vertex_to_faces[idx1].push(face_idx);
        vertex_to_faces[idx2].push(face_idx);
    }

    // 2. Пересчет координат каждой вершины через Метод Наименьших Квадратов (LSM)
    let mut new_vertices = Vec::with_capacity(vertices.len());

    for (v_idx, &orig_vertex) in vertices.iter().enumerate() {
        let connected_faces = &vertex_to_faces[v_idx];
        
        if connected_faces.is_empty() {
            new_vertices.push(orig_vertex);
            continue;
        }

        let mut avg_normal = Vec3::ZERO;
        let mut ata = Mat3::ZERO;
        let mut atb = Vec3::ZERO;

        for &f_idx in connected_faces {
            let plane = &face_planes[f_idx];
            let n = plane.normal;
            avg_normal += n;
            
            // Тензорное (внешнее) произведение n * n^T (строго по колонкам для glam матрицы 3x3)
            let n_outer = Mat3::from_cols(
                Vec3::new(n.x * n.x, n.x * n.y, n.x * n.z), // Column 0
                Vec3::new(n.y * n.x, n.y * n.y, n.y * n.z), // Column 1
                Vec3::new(n.z * n.x, n.z * n.y, n.z * n.z), // Column 2
            );
            
            ata += n_outer;
            atb += n * plane.d;
        }

        // Надежный вектор смещения для вырожденных участков
        let fallback_normal = if avg_normal.length_squared() > 1e-12 {
            avg_normal.normalize()
        } else {
            Vec3::Z
        };

        // Проверяем определитель матрицы ATA на полноту ранга (> 0)
        if ata.determinant().abs() > 1e-8 {
            let ata_inv = ata.inverse();
            let new_coords = ata_inv * atb;
            
            // Фильтр от геометрических «выбросов» (острые ребра с углами менее 1-2 градусов)
            if (new_coords - orig_vertex).length() > offset * 4.0 {
                new_vertices.push(orig_vertex + fallback_normal * offset);
            } else {
                new_vertices.push(new_coords);
            }
        } else {
            // Если плоскости параллельны (плоский борт, палуба или днище), смещаем строго по средней нормали
            new_vertices.push(orig_vertex + fallback_normal * offset);
        }
    }

    // 3. Сборка нового TriMesh для Parry3d
    // Передаем TriMeshFlags::empty() чтобы избежать повторного построения дорогостоящей топологии
    TriMesh::with_flags(new_vertices, indices.to_vec(), TriMeshFlags::all())
        .expect("Ошибка сборки TriMesh: нарушена топология сетки")
}