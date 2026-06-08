use super::{Hydrostatics, Plane};
use parry3d_f64::math::Vec3;

pub struct SlicedMesh {
    /// Треугольники, оказавшиеся под плоскостью ватерлинии (погруженный объем)
    pub submerged_triangles: Vec<[Vec3; 3]>,
    /// Отрезки, формирующие контур сечения (ватерлинию)
    pub waterline_edges: Vec<[Vec3; 2]>,
}

impl SlicedMesh {
    pub fn volume(&self) -> f64 {
        self.hydrostatics().volume
    }

    pub fn hydrostatics(&self) -> Hydrostatics {
        if self.submerged_triangles.is_empty() {
            return Hydrostatics {
                volume: 0.,
                center_of_buoyancy: Vec3::ZERO,
            };
        }

        let mut total_volume = 0.0;
        let mut sum_centroid = Vec3::ZERO;

        let p_ref = self
            .waterline_edges
            .first()
            .map(|edge| edge[0])
            .unwrap_or_else(|| self.submerged_triangles.first().unwrap()[0]);

        for tri in &self.submerged_triangles {
            let p0 = tri[0];
            let p1 = tri[1];
            let p2 = tri[2];

            let a = p0 - p_ref;
            let b = p1 - p_ref;
            let c = p2 - p_ref;

            let v_i = a.dot(b.cross(c)) / 6.0;
            total_volume += v_i;

            let centroid_i = (p0 + p1 + p2 + p_ref) * 0.25;
            sum_centroid += centroid_i * v_i;
        }

        let abs_volume = total_volume.abs();
        let center_of_buoyancy = if abs_volume > f64::EPSILON {
            sum_centroid / total_volume
        } else {
            Vec3::ZERO
        };

        Hydrostatics {
            volume: abs_volume,
            center_of_buoyancy,
        }
    }

    pub fn calculate_waterline_properties(&self) -> (f64, Vec3) {
        let edges = &self.waterline_edges;
        if edges.is_empty() {
            return (0.0, Vec3::ZERO);
        }

        let mut sum_pts = Vec3::ZERO;
        for edge in edges {
            sum_pts += edge[0] + edge[1];
        }
        let origin = sum_pts / (edges.len() * 2) as f64;

        let mut signed_area_2x = 0.0;
        let mut moment_x = 0.0;
        let mut moment_y = 0.0;
        let mut z_sum = 0.0;

        for edge in edges {
            let p1 = edge[0] - origin;
            let p2 = edge[1] - origin;

            let det = p1.x * p2.y - p2.x * p1.y;
            signed_area_2x += det;

            moment_x += (p1.x + p2.x) * det;
            moment_y += (p1.y + p2.y) * det;
            z_sum += p1.z + p2.z;
        }

        let area = signed_area_2x.abs() / 2.0;
        if area < 1e-10 {
            return (0.0, origin);
        }

        let cx = moment_x / (3.0 * signed_area_2x);
        let cy = moment_y / (3.0 * signed_area_2x);
        let cz = z_sum / (edges.len() * 2) as f64;

        let centroid = Vec3::new(origin.x + cx, origin.y + cy, origin.z + cz);
        (area, centroid)
    }

    pub fn inertia(&self) -> (f64, f64) {
        let edges = &self.waterline_edges;
        if edges.is_empty() {
            return (0.0, 0.0);
        }

        let mut area = 0.0;
        let mut sx = 0.0;
        let mut sy = 0.0;
        let mut ix_0 = 0.0;
        let mut iy_0 = 0.0;

        for edge in edges {
            let (x1, y1) = (edge[0].x, edge[0].y);
            let (x2, y2) = (edge[1].x, edge[1].y);

            let f = x1 * y2 - x2 * y1;
            area += f;
            sx += (y1 + y2) * f;
            sy += (x1 + x2) * f;
            ix_0 += (y1 * y1 + y1 * y2 + y2 * y2) * f;
            iy_0 += (x1 * x1 + x1 * x2 + x2 * x2) * f;
        }

        let area = area / 2.0;
        if area.abs() < 1e-10 { return (0.0, 0.0); }

        let cy = sx / (6.0 * area);
        let cx = sy / (6.0 * area);

        let ix = (ix_0 / 12.0).abs() - area.abs() * cy * cy;
        let iy = (iy_0 / 12.0).abs() - area.abs() * cx * cx;

        (ix, iy)
    }

    pub fn waterline_size(&self) -> (f64, f64) {
        let (min_p, max_p) = self.waterline_edges
            .iter()
            .flat_map(|edge| [edge[0], edge[1]]) 
            .fold(
                ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)),

                |((min_x, min_y), (max_x, max_y)), p| {
                    (
                        (min_x.min(p.x), min_y.min(p.y)),
                        (max_x.max(p.x), max_y.max(p.y)),
                    )
                },
            );

        if min_p.0 == f64::MAX {
            (0.0, 0.0)
        } else {
            (max_p.0 - min_p.0, max_p.1 - min_p.1)
        }
    }

    /// Вычисляет характеристики погруженного поперечного сечения (шпангоута) на координате X.
    /// Возвращает: (Площадь сечения, Погруженная ширина B, Погруженная высота T)
    pub fn calculate_cross_section(&self, x_coord: f64) -> (f64, f64, f64) {
        if self.submerged_triangles.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let section_plane = Plane {
            normal: Vec3::X,
            d: x_coord,
        };

        // Сечем подводные грани. 
        // Локальные 2D оси: U = Y (ширина), V = Z (высота шпангоута)
        let section_2d = section_plane.slice_triangles(
            &self.submerged_triangles, 
            Vec3::Y, 
            Vec3::Z
        );

        let area = section_2d.calculate_area();
        let (width, height) = section_2d.size();

        (area, width, height)
    }
}
