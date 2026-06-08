use baby_shark::decimation::{ConstantErrorDecimationCriteria, EdgeDecimator};
use baby_shark::exports::nalgebra::Vector3;
use baby_shark::mesh::corner_table::{CornerTableD, VertexId};
use parry3d_f64::math::Vec3;
use parry3d_f64::shape::{TriMesh, TriMeshFlags};
use rustc_hash::{FxBuildHasher, FxHashMap};
use sal_core::error::Error;


pub fn optimize_binary_stl(mesh: &TriMesh, min_delta: f64) -> Result<TriMesh, Error> {
    let mut mesh = trimesh_to_baby_shark(mesh);
    let decimation_criteria = ConstantErrorDecimationCriteria::new(min_delta);
    let mut decimator = EdgeDecimator::new()
        .decimation_criteria(decimation_criteria)
        .keep_boundary(true);
    decimator.decimate(&mut mesh);
    baby_shark_to_trimesh(&mesh)
}

pub fn optimize_trimesh(mesh: &TriMesh, min_delta: f64) -> Result<TriMesh, Error> {
    let mut mesh = trimesh_to_baby_shark(mesh);
    let decimation_criteria = ConstantErrorDecimationCriteria::new(min_delta);
    let mut decimator = EdgeDecimator::new()
        .decimation_criteria(decimation_criteria)
        .keep_boundary(true);
    decimator.decimate(&mut mesh);
    baby_shark_to_trimesh(&mesh)
}

/// Конвертирует сетку из формата tri-mesh в структуру CornerTable (baby_shark)
pub fn trimesh_to_baby_shark(source: &TriMesh) -> CornerTableD {
    // 1. Извлекаем вершины и приводим их к типу Vector3 <f32>
    let vertices: Vec<Vector3<f64>> = source
        .vertices()
        .iter()
        .map(|v| Vector3::new(v.x, v.y, v.z))
        .collect();

    // 2. Извлекаем индексы треугольников и разворачиваем в плоский массив
    let indices: Vec<usize> = source
        .indices()
        .iter()
        .flat_map(|&[a, b, c]| vec![a as usize, b as usize, c as usize])
        .collect();

    // 3. Создаем CornerTableD
    CornerTableD::from_vertex_and_face_slices(&vertices, &indices)
}
/// Конвертирует сетку из структуры CornerTable (baby_shark) обратно в tri-mesh
pub fn baby_shark_to_trimesh(corner_table: &CornerTableD) -> Result<TriMesh, Error> {
    let error = Error::new("optimization", "baby_shark_to_trimesh");
    // 1. Выделяем память под вершины.
    // Parry ожидает Vec из Point3 (в nalgebra это алиас для OPoint<f64, Const<3>>)
    let mut vertices: Vec<Vec3> = Vec::with_capacity(corner_table.count_vertices());

    // Хеш-карта для сопоставления старого VertexId и нового последовательного индекса (от 0 до N)
    let mut vertex_mapping: FxHashMap<VertexId, u32> = FxHashMap::with_capacity_and_hasher(
        corner_table.vertices().count(),
        FxBuildHasher::default(),
    );
    let mut current_new_idx = 0_u32;

    // Внутренняя адресация в CornerTable идет по числовым индексам.
    // Проверим каждую вершину от 0 до максимально возможной.
    for v_id in corner_table.vertices() {
        // В процессе децимации вершины удаляются. Нам нужны только живые.
        // Проверим, существует ли вершина и не удалена ли она
        let position = corner_table.vertex_position(v_id);

        // Создаем точку nalgebra Point3/OPoint и добавляем её в вектор
        let point = Vec3::from([position.x as f64, position.y as f64, position.z as f64]);
        vertices.push(point);

        // Мапим старый VertexId на новый индекс от 0 до N
        vertex_mapping.insert(v_id, current_new_idx);
        current_new_idx += 1;
    }

    // 2. Выделяем память под индексы
    let mut indices: Vec<[u32; 3]> = Vec::with_capacity(corner_table.count_faces());

    // По аналогии итерируемся по всем возможным граням
    for f_id in corner_table.faces() {
        let (f1, f2, f3) = corner_table.face_vertices(f_id);

        // Находим их новые порядковые индексы в нашей хеш-карте
        let idx1 = *vertex_mapping
            .get(&f1)
            .ok_or(error.err("Vertex mapping is broken"))?;
        let idx2 = *vertex_mapping
            .get(&f2)
            .ok_or(error.err("Vertex mapping is broken"))?;
        let idx3 = *vertex_mapping
            .get(&f3)
            .ok_or(error.err("Vertex mapping is broken"))?;

        // Записываем треугольник как массив из 3 элементов
        indices.push([idx1, idx2, idx3]);
    }

    // 3. Создаем TriMesh из полученных типизированных векторов.
    // Parry возвращает Result, поэтому в конце мы используем .expect() или ?
    TriMesh::with_flags(vertices, indices, TriMeshFlags::all())
        .map_err(|err| error.pass_with("Failed to create Parry TriMesh", format!("{}", err)))
}
