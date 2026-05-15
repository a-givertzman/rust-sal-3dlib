mod hydrostatic;
mod strength;
mod windage;

pub use hydrostatic::*;
pub use strength::*;
pub use windage::*;

use parry3d_f64::math::{Mat3, Vec3};
use parry3d_f64::shape::{TriMesh, TriMeshFlags};
use sal_3dlib_core::math::Position;

/// полный объем модели
pub fn properties(mesh: &TriMesh, density: f64) -> (f64, Position) {
    let properties = parry3d_f64::shape::Shape::mass_properties(mesh, density);
    let mass = if properties.inv_mass > 0. { 1. / properties.inv_mass } else { 0. };
    (
        mass,
        Position::new(
            properties.local_com.x,
            -properties.local_com.y,
            properties.local_com.z,
        ),
    )
}
/// Объем меша
pub fn volume(mesh: &TriMesh) -> f64 {
    let inv_mass = parry3d_f64::shape::Shape::mass_properties(mesh, 1.).inv_mass;
    if inv_mass > 0. { 1. / inv_mass } else { 0. }
}
///
/// Расчет начала координат для отсеков как
/// проекции центра объема модели на ее нижнюю плоскость
pub fn compartment_center(mesh: &TriMesh) -> Vec3 {
    let properties = parry3d_f64::shape::Shape::mass_properties(mesh, 1.);
    let aabb = mesh.local_aabb();
    Vec3::new(properties.local_com.x, properties.local_com.y, aabb.mins.z)
}

/// Разбиение меша по высоте на draught_qnt_steps шагов.
/// Макимальный и минимальный уровень считаются с учетом наклона
pub fn draught_steps(
    mesh: &TriMesh,
    center: Vec3,
    level_step_qnt: usize,
    max_heel: f64,
    max_trim: f64,
) -> (f64, Vec<f64>) {
    assert!(level_step_qnt > 2);
    let aabb = mesh.local_aabb();
    assert!(aabb.maxs.x >= center.x);
    assert!(aabb.mins.x <= center.x);
    assert!(aabb.maxs.y >= center.y);
    assert!(aabb.mins.y <= center.y);
    let mut result = vec![];
    let max_dx = (aabb.maxs.x - center.x).max(center.x - aabb.mins.x);
    let max_dy = (aabb.maxs.y - center.y).max(center.y - aabb.mins.y);
    let max_dz = max_dx * max_trim.to_radians().tan().abs()
        + max_dy * max_heel.to_radians().tan().abs() * max_trim.to_radians().cos();
    let delta_z = aabb.maxs.z - aabb.mins.z;
    let min_z = -max_dz;
    let max_z = delta_z + max_dz;
    let step = delta_z / (level_step_qnt as f64 - 1.);
    let step_max_dz = if max_dz * 2. > delta_z {
        2. * max_dz / (level_step_qnt as f64 - 1.)
    } else {
        step
    };
    let mut current = min_z;
    let mut current_step = step_max_dz;
    while current + current_step / 2. <= 0. {
        result.push(current);
        current += current_step;
    }
    current = 0.;
    current_step = step;
    while current + current_step / 2. <= delta_z {
        result.push(current);
        current += current_step;
    }
    current = delta_z;
    current_step = step_max_dz;
    while current + current_step / 2. <= max_z {
        result.push(current);
        current += current_step;
    }
    result.push(max_z);
    //  log::debug!("shape draught_steps max_dx:{} max_dy:{} max_dz:{} delta_z:{} step_max_dz:{} min_z:{} max_z:{} aabb.mins.z:{} aabb.maxs.z:{} level_step_qnt:{}",
    //      max_dx, max_dy, max_dz, delta_z, step_max_dz, min_z, max_z, aabb.mins.z, aabb.maxs.z, level_step_qnt);
    //  log::debug!("shape draught_steps {:?}", result);
    (aabb.mins.z, result)
}

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

        // В parry3d 0.26 вершины — это напрямую glam::Vec3, касты не требуются
        let p0 = vertices[idx0];
        let p1 = vertices[idx1];
        let p2 = vertices[idx2];

        let v01 = p1 - p0;
        let v02 = p2 - p0;
        
        let mut normal = v02.cross(v01);
        if normal.length_squared() > 1e-6 {
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
        let fallback_normal = if avg_normal.length_squared() > 1e-6 {
            avg_normal.normalize()
        } else {
            Vec3::Z
        };

        // Проверяем определитель матрицы ATA на полноту ранга (> 0)
        if ata.determinant().abs() > 1e-4 {
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
    TriMesh::with_flags(new_vertices, indices.to_vec(), TriMeshFlags::empty())
        .expect("Ошибка сборки TriMesh: нарушена топология сетки")
}