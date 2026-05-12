use parry3d_f64::{bounding_volume::Aabb, math::Vec3, shape::TriMesh};
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
    let aabb: Aabb = mesh.local_aabb();
    Vec3::new(properties.local_com.x, properties.local_com.y, aabb.mins.z)
}

