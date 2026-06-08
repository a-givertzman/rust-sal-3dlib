use super::SlicedMesh;
use parry3d_f64::{math::*, shape::TriMesh};

#[derive(Clone, Debug)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f64,
}

/// Результат сечения 3D-геометрии плоскостью, спроецированный в 2D-координаты
#[derive(Clone, Debug)]
pub struct PlanarSection2D {
    /// Отрезки в локальных координатах (u, v) плоскости
    pub edges: Vec<[[f64; 2]; 2]>,
    /// Границы сечения в локальных координатах: ((min_u, min_v), (max_u, max_v))
    pub bounds: ((f64, f64), (f64, f64)),
}

impl Plane {
    pub fn from_point_and_normal(point: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        let d = normal.dot(point);
        Self { normal, d }
    }

    #[inline(always)]
    pub fn distance(&self, point: &Vec3) -> f64 {
        self.normal.dot(*point) - self.d
    }

    pub fn slice_mesh(&self, mesh: &TriMesh) -> SlicedMesh {
        let vertices = mesh.vertices();
        let indices = mesh.indices();

        let distances: Vec<f64> = vertices
            .iter()
            .map(|v| self.distance(&Vec3::new(v.x, v.y, v.z)))
            .collect();

        let mut submerged_triangles = Vec::with_capacity(indices.len());
        let mut waterline_edges = Vec::new();

        for face in indices {
            let idx = [face[0] as usize, face[1] as usize, face[2] as usize];
            let d = [distances[idx[0]], distances[idx[1]], distances[idx[2]]];
            let v = [
                Vec3::new(vertices[idx[0]].x, vertices[idx[0]].y, vertices[idx[0]].z),
                Vec3::new(vertices[idx[1]].x, vertices[idx[1]].y, vertices[idx[1]].z),
                Vec3::new(vertices[idx[2]].x, vertices[idx[2]].y, vertices[idx[2]].z),
            ];

            let above_mask = [d[0] > 0.0, d[1] > 0.0, d[2] > 0.0];
            let above_count = above_mask.iter().filter(|&&a| a).count();

            match above_count {
                0 => {
                    submerged_triangles.push([v[0], v[1], v[2]]);
                }
                3 => {}
                1 => {
                    let i0 = above_mask.iter().position(|&a| a).unwrap();
                    let i1 = (i0 + 1) % 3;
                    let i2 = (i0 + 2) % 3;

                    let p1 = intersect_edge(&v[i0], &v[i1], d[i0], d[i1]);
                    let p2 = intersect_edge(&v[i0], &v[i2], d[i0], d[i2]);

                    submerged_triangles.push([v[i1], v[i2], p1]);
                    submerged_triangles.push([v[i2], p2, p1]);

                    waterline_edges.push([p1, p2]);
                }
                2 => {
                    let i0 = above_mask.iter().position(|&a| !a).unwrap();
                    let i1 = (i0 + 1) % 3;
                    let i2 = (i0 + 2) % 3;

                    let p1 = intersect_edge(&v[i0], &v[i1], d[i0], d[i1]);
                    let p2 = intersect_edge(&v[i0], &v[i2], d[i0], d[i2]);

                    submerged_triangles.push([v[i0], p1, p2]);

                    waterline_edges.push([p2, p1]);
                }
                _ => unreachable!(),
            }
        }

        SlicedMesh {
            submerged_triangles,
            waterline_edges,
        }
    }

    /// Сечет массив 3D-треугольников и проецирует отрезки пересечения в локальные 2D-координаты (u, v)
    pub fn slice_triangles(
        &self,
        triangles: &[[Vec3; 3]],
        u_axis: Vec3,
        v_axis: Vec3,
    ) -> PlanarSection2D {
        let mut edges = Vec::new();
        let mut min_u = f64::MAX;
        let mut min_v = f64::MAX;
        let mut max_u = f64::MIN;
        let mut max_v = f64::MIN;

        let project = |p: Vec3| [p.dot(u_axis), p.dot(v_axis)];

        let mut update_bounds = |u: f64, v: f64| {
            if u < min_u {
                min_u = u;
            }
            if u > max_u {
                max_u = u;
            }
            if v < min_v {
                min_v = v;
            }
            if v > max_v {
                max_v = v;
            }
        };

        for tri in triangles {
            let d = [
                self.distance(&tri[0]),
                self.distance(&tri[1]),
                self.distance(&tri[2]),
            ];
            let above_mask = [d[0] > 0.0, d[1] > 0.0, d[2] > 0.0];
            let above_count = above_mask.iter().filter(|&&a| a).count();

            if above_count == 0 || above_count == 3 {
                continue;
            }

            let (p1_3d, p2_3d) = if above_count == 1 {
                let i0 = above_mask.iter().position(|&a| a).unwrap();
                let i1 = (i0 + 1) % 3;
                let i2 = (i0 + 2) % 3;
                (
                    intersect_edge(&tri[i0], &tri[i1], d[i0], d[i1]),
                    intersect_edge(&tri[i0], &tri[i2], d[i0], d[i2]),
                )
            } else {
                let i0 = above_mask.iter().position(|&a| !a).unwrap();
                let i1 = (i0 + 1) % 3;
                let i2 = (i0 + 2) % 3;
                (
                    intersect_edge(&tri[i0], &tri[i2], d[i0], d[i2]),
                    intersect_edge(&tri[i0], &tri[i1], d[i0], d[i1]),
                )
            };

            let uv1 = project(p1_3d);
            let uv2 = project(p2_3d);

            update_bounds(uv1[0], uv1[1]);
            update_bounds(uv2[0], uv2[1]);

            edges.push([uv1, uv2]);
        }

        PlanarSection2D {
            edges,
            bounds: if min_u == f64::MAX {
                ((0., 0.), (0., 0.))
            } else {
                ((min_u, min_v), (max_u, max_v))
            },
        }
    }
    /// Универсальный безаллокационный расчет геометрических характеристик плоского сечения.
    /// Вычисляет площадь по формуле знаковых трапеций: (u1 - u2) * (v1 + v2) * 0.5
    /// `u_axis` — координатная ось (продольная или поперечная), `v_axis` — ось высоты/глубины.
    pub fn compute_section_properties(
        &self,
        triangles: &[[Vec3; 3]],
        water_plane: &Plane, // Передаем плоскость ватерлинии для расчета глубин
        u_axis: Vec3,
        v_axis: Vec3,
    ) -> (f64, f64, f64) {
        let mut total_area = 0.0;
        let mut min_u = f64::MAX;
        let mut min_v = f64::MAX;
        let mut max_u = f64::MIN;
        let mut max_v = f64::MIN;

        #[inline(always)]
        fn project(p: Vec3, u_axis: Vec3, v_axis: Vec3) -> (f64, f64) {
            (p.dot(u_axis), p.dot(v_axis))
        }

        for tri in triangles {
            // ШАГ 1: Считаем знаковое расстояние от вершин треугольника до плоскости сечения
            let mut d = [
                self.distance(&tri[0]),
                self.distance(&tri[1]),
                self.distance(&tri[2]),
            ];

            // ЗАЩИТА ОТ МАШИННОГО НУЛЯ
            for val in d.iter_mut() {
                if val.abs() < 1e-9 {
                    *val = 1e-9;
                }
            }

            // ШАГ 2: Маска знаков для классификации пересечения
            let above_mask = [d[0] > 0.0, d[1] > 0.0, d[2] > 0.0];
            let above_count = above_mask.iter().filter(|&&a| a).count();

            if above_count == 0 || above_count == 3 {
                continue;
            }

            // ШАГ 3: Линейная интерполяция 3D-точек пересечения на ребрах шпангоута/батокса
            let (p1_3d, p2_3d) = if above_count == 1 {
                let i0 = above_mask.iter().position(|&a| a).unwrap();
                let i1 = (i0 + 1) % 3;
                let i2 = (i0 + 2) % 3;
                (
                    intersect_edge(&tri[i0], &tri[i1], d[i0], d[i1]),
                    intersect_edge(&tri[i0], &tri[i2], d[i0], d[i2]),
                )
            } else {
                let i0 = above_mask.iter().position(|&a| !a).unwrap();
                let i1 = (i0 + 1) % 3;
                let i2 = (i0 + 2) % 3;
                (
                    intersect_edge(&tri[i0], &tri[i2], d[i0], d[i2]),
                    intersect_edge(&tri[i0], &tri[i1], d[i0], d[i1]),
                )
            };

            // ШАГ 4: Перевод в плоские 2D-координаты и проверка 3D-нормали треугольника
            let (mut u1, mut v1) = project(p1_3d, u_axis, v_axis);
            let (mut u2, mut v2) = project(p2_3d, u_axis, v_axis);

            let tri_normal = (tri[1] - tri[0]).cross(tri[2] - tri[0]);
            let is_direct_order = tri_normal.dot(v_axis) >= 0.0;

            // Настраиваем порядок точек отрезка по горизонтальной оси U (правило знаков по Х)
            if is_direct_order {
                if u1 < u2 {
                    std::mem::swap(&mut u1, &mut u2);
                    std::mem::swap(&mut v1, &mut v2);
                }
            } else {
                if u1 > u2 {
                    std::mem::swap(&mut u1, &mut u2);
                    std::mem::swap(&mut v1, &mut v2);
                }
            }

            if u1 < min_u {
                min_u = u1;
            }
            if u1 > max_u {
                max_u = u1;
            }
            if v1 < min_v {
                min_v = v1;
            }
            if v1 > max_v {
                max_v = v1;
            }
            if u2 < min_u {
                min_u = u2;
            }
            if u2 > max_u {
                max_u = u2;
            }
            if v2 < min_v {
                min_v = v2;
            }
            if v2 > max_v {
                max_v = v2;
            }

            // ШАГ 5: Расчет ЧИСТОЙ ФИЗИЧЕСКОЙ ВЫСОТЫ трапеции через 3D-плоскость ватерлинии.
            // Так как точки под водой, distance вернет отрицательное число, берем с минусом.
            let height1 = -water_plane.distance(&p1_3d);
            let height2 = -water_plane.distance(&p2_3d);

            // Ваша чистая формула знаковых трапеций
            let step_area = (u1 - u2) * (height1 + height2) * 0.5;
            total_area += step_area;
        }

        let width = if min_u == f64::MAX {
            0.0
        } else {
            max_u - min_u
        };
        let height = if min_v == f64::MAX {
            0.0
        } else {
            max_v - min_v
        };

        (total_area.abs(), width, height)
    }
}

#[inline(always)]
pub fn intersect_edge(a: &Vec3, b: &Vec3, d_a: f64, d_b: f64) -> Vec3 {
    let denom = d_a - d_b;
    if denom.abs() < 1e-8 {
        return *a;
    }
    let t = d_a / denom;
    a + (b - a) * t
}

impl PlanarSection2D {
    pub fn calculate_area(&self) -> f64 {
        if self.edges.is_empty() {
            return 0.0;
        }

        let mut total_area = 0.0;

        for edge in &self.edges {
            let p1 = edge[0]; // [u1, v1]
            let p2 = edge[1]; // [u2, v2]

            // Вертикальный шаг отрезка шпангоута (высота полосы по оси V / Z)
            let delta_v = p1[1] - p2[1];

            // Средняя ширина этого участка шпангоута по оси U (ось Y судна).
            // Берем модуль координат, чтобы левый (+Y) и правый (-Y) борт давали
            // исключительно положительный вклад в ширину и не вычитались!
            let avg_u = (p1[0].abs() + p2[0].abs()) * 0.5;

            // Площадь элементарной трапеции, образуемой отрезком борта с диаметральной плоскостью (U=0)
            let strip_area = avg_u * delta_v;

            // Суммируем абсолютное значение площади полосы
            total_area += strip_area.abs();
        }

        total_area
    }
    //
    pub fn size(&self) -> (f64, f64) {
        let ((min_u, min_v), (max_u, max_v)) = self.bounds;
        if min_u == f64::MAX {
            (0.0, 0.0)
        } else {
            (max_u - min_u, max_v - min_v)
        }
    }
}
