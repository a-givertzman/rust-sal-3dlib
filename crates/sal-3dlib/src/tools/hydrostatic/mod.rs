mod hydrostatics;
mod plane;
mod sliced_mesh;

pub use hydrostatics::*;
pub use plane::*;
pub use sliced_mesh::*;

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

pub fn calculate_h_slant(mesh: &TriMesh, origin: Vec3, heel: f64, trim: f64) -> f64 {
    let isometry = position(&origin, heel, trim, 0.);
    let array: Vec<_> = mesh.vertices().iter().map(|p| isometry.transform_point(Vec3::new(p.x, p.y, p.z))).collect();
    let p_min = array.into_iter()
        .fold(Vec3::new(0., 0., f64::MAX), |p_min, p_current| {
            if p_min.z > p_current.z {
                p_current
            } else {
                p_min
            }
        });
    #[allow(non_snake_case)]
    let [X, Y, Z] = p_min.to_array();
    let [xi, yi, _] = origin.to_array();    
    let theta = heel.to_radians();
    let psi = trim.to_radians();
    let res = Z + (yi-Y)*theta.tan() + (xi-X)*psi.tan()/theta.cos();
    res
}

pub fn position(center: &Vec3, heel: f64, trim: f64, draught: f64) -> Pose3 {
    let heel_rad = heel.to_radians();
    let trim_rad = trim.to_radians();

    let trim_rotation = DQuat::from_axis_angle(Vec3::Y, trim_rad);
    let transformed_x_axis = trim_rotation * Vec3::X;
    let heel_rotation = DQuat::from_axis_angle(transformed_x_axis.normalize(), heel_rad);
    let rotation = heel_rotation * trim_rotation;

    let mut center_offset = *center;
    center_offset.z += draught;

    let point = rotation * center_offset;
    Pose3::from_parts(-point, rotation)
}
/// Вычисляет характеристики поперечного вертикального сечения (диаметральной плоскостью) погруженной части корпуса.
/// `x_coord` — смещение по X
/// возвращает [площадь, ширина, высота]
pub fn calculate_cross_section(mesh: &TriMesh, x_coord: f64, draught: f64) -> (f64, f64, f64) {
    let water_plane = Plane::from_point_and_normal(Vec3::new(0., 0., draught), Vec3::Z);
    let sliced_mesh = water_plane.slice_mesh(mesh);
    let cross_plane = Plane::from_point_and_normal(Vec3::new(x_coord, 0., 0.), Vec3::X);    
    cross_plane.compute_section_properties(
        &sliced_mesh.submerged_triangles, 
        &water_plane,
        Vec3::Y, 
        Vec3::Z,
    )
}
/// Вычисляет характеристики продольного вертикального сечения (батокса) погруженной части корпуса.
/// `y_coord` — смещение по Y
/// возвращает [площадь, длина, высота]
pub fn calculate_buttock_section(mesh: &TriMesh, y_coord: f64, draught: f64) -> (f64, f64, f64) {
    let water_plane = Plane::from_point_and_normal(Vec3::new(0., 0., draught), Vec3::Z);
    let sliced_mesh = water_plane.slice_mesh(mesh);
    let cross_plane = Plane::from_point_and_normal(Vec3::new(0., y_coord, 0.), Vec3::Y);    
    cross_plane.compute_section_properties(
        &sliced_mesh.submerged_triangles, 
        &water_plane,
        Vec3::X, 
        Vec3::Z,
    )
}
/// Вычисляет размер погруженной части корпуса
/// возвращает (dx, dy, dz)
pub fn calculate_aabb(mesh: &TriMesh, center: Vec3, heel: f64, trim: f64, draught: f64) -> (f64, f64, f64) {
    let isometry = position(&center, heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);
    let sliced_mesh = plane.slice_mesh(mesh);
    sliced_mesh.aabb()
}
///
/// Поиск пересечения плоскости и ломанной кривой, состоящей из отрезков
/// на выходе вектор пересечений, пересечений может быть несколько
/// если пересечений нет пустой вектор
pub fn get_cross(point: Vec3, normal: Vec3, edges: &Vec<[Vec3; 2]>) -> Vec<Vec3> {
    let mut intersections = Vec::new();
    let normal_normalized = normal.normalize();
    for edge in edges {
        let p1 = edge[0];
        let p2 = edge[1];
        let direction = p2 - p1;
        let dot_denominator = direction.dot(normal_normalized);
        if dot_denominator.abs() > 1e-6 {
            let t = (point - p1).dot(normal_normalized) / dot_denominator;
            if (0.0..=1.0).contains(&t) {
                let intersection_point = p1 + t * direction;
                intersections.push(intersection_point);
            }
        }
    }
    intersections
}

