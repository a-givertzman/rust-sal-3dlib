mod hydrostatics;
mod plane;
mod sliced_mesh;

pub use sliced_mesh::*;
pub use plane::*;
pub use hydrostatics::*;

use parry3d_f64::glamx::DQuat;
use parry3d_f64::math::*;
use parry3d_f64::shape::TriMesh;

pub fn calculate_hydrostatic(mesh: &TriMesh, center: Vec3, heel: f64, trim: f64, draught: f64) -> (f64, Vec3) {
    let isometry = position(&center, heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);
    let sliced_mesh = plane.slice_mesh(mesh);
    let hydrostatics = sliced_mesh.hydrostatics();
    let center = hydrostatics.center_of_buoyancy;
    let center = Vec3::new(center.x, -center.y, center.z);
    (hydrostatics.volume, center)
}

pub fn calculate_waterline(mesh: &TriMesh, center: Vec3, heel: f64, trim: f64, draught: f64) -> (f64, Vec3) {
    let isometry = position(&center, heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);

    let mut sliced_mesh = plane.slice_mesh(mesh);

    let isometry = isometry.inverse();
    sliced_mesh.waterline_edges = sliced_mesh.waterline_edges.iter()
       .map(|v| [isometry.transform_point(v[0]), isometry.transform_point(v[1])] ).collect();
    let (area, area_center) = sliced_mesh.calculate_waterline_properties();
    let center = isometry.inverse_transform_point(area_center);
    let center = Vec3::new(center.x, -center.y, center.z);
    (area, center)
}

pub fn calculate_inertia(mesh: &TriMesh, center: Vec3, heel: f64, trim: f64, draught: f64) -> (f64, f64) {
    let isometry = position(&center, heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);
    let mut sliced_mesh = plane.slice_mesh(mesh);
    let isometry = isometry.inverse();
    sliced_mesh.waterline_edges = sliced_mesh.waterline_edges.iter()
       .map(|v| [isometry.transform_point(v[0]), isometry.transform_point(v[1])] ).collect();
    let (ix, iy) = sliced_mesh.inertia();
    (ix, iy)
}

pub fn calculate_waterline_size(mesh: &TriMesh, draught: f64) -> (f64, f64) {
    let plane = Plane::from_point_and_normal(Vec3::new(0., 0., draught), Vec3::Z);
    let sliced_mesh = plane.slice_mesh(mesh);
    let (dx, dy) = sliced_mesh.waterline_size();
    (dx, dy)
}
//
pub fn position(center: &Vec3, heel: f64, trim: f64, draught: f64) -> Pose3 {
    let heel_rad = heel.to_radians();
    let trim_rad = trim.to_radians();

    // 1. Вращение по дифференту (trim) вокруг оси Y
    let trim_rotation = DQuat::from_axis_angle(Vec3::Y, trim_rad);

    // 2. Находим трансформированную ось X для крена (heel)
    let transformed_x_axis = trim_rotation * Vec3::X;
    
    // 3. Вращение по крену вокруг новой оси X
    let heel_rotation = DQuat::from_axis_angle(transformed_x_axis.normalize(), heel_rad);
    
    // Итоговое вращение
    let rotation = heel_rotation * trim_rotation;

    // 4. Смещение центра
    let mut center_offset = *center;
    center_offset.z += draught;

    // 5. Вычисляем позицию (в glam вращение точки делается через оператор *)
    let point = rotation * center_offset;

    // 6. Создаем Isometry (в Parry с фичей glam это структура с полями translation и rotation)
    Pose3::from_parts(-point, rotation)
}

