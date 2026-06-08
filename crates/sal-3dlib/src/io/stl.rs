use std::io::Cursor;
use sal_core::error::Error;

/// Парсинг из массива бинарных данных в формате бинарного stl,
/// возвращает массив вершин + массив индексов
pub fn parse(data: &[u8]) -> Result<(Vec<[f32; 3]>, Vec<[usize; 3]>), Error> {
    let error = Error::new("io.stl", "parse");
    let cursor = Cursor::new(&data);
    let mut reader = std::io::BufReader::new(cursor);
    
    let stl_mesh = stl_io::read_stl(&mut reader)
        .map_err(|err| error.pass_with("stl_io::read_stl", err.to_string()))?;

    // 1. Выгружаем вершины
    let vertices = stl_mesh
        .vertices
        .iter()
        .map(|v| v.0)
        .collect::<Vec<_>>();

    // 2. Формируем массив индексов
    let indices = stl_mesh
        .faces
        .into_iter()
        .map(|f| f.vertices)
        .collect::<Vec<_>>();
    
    Ok((vertices, indices))
}
/// Сериализация в массив бинарных данных в формате бинарного stl
pub fn serialize(vertices: &[[f32; 3]], indices: &[[usize; 3]]) -> Result<Vec<u8>, Error> {
    let error = Error::new("io.stl", "serialize_stl");

    let mut triangles = Vec::with_capacity(indices.len());

    for idx in indices {
        // 1. Безопасно извлекаем индексы вершин (избегаем panic при некорректных данных)
        let i0 = idx[0];
        let i1 = idx[1];
        let i2 = idx[2];

        // Проверяем, что индексы не выходят за границы массива вершин
        if i0 >= vertices.len() || i1 >= vertices.len() || i2 >= vertices.len() {
            return Err(error.err(format!(
                "Index out of bounds: vertices len {}, but indices are [{}, {}, {}]",
                vertices.len(), i0, i1, i2
            )));
        }

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        // 2. Векторы сторон треугольника: u = v1 - v0, v = v2 - v0
        let u = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let v = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        // 3. Вычисление векторного произведения (Cross Product) для получения перпендикуляра
        let mut nx = u[1] * v[2] - u[2] * v[1];
        let mut ny = u[2] * v[0] - u[0] * v[2];
        let mut nz = u[0] * v[1] - u[1] * v[0];

        // 4. Нормализация вектора нормали
        let length = (nx * nx + ny * ny + nz * nz).sqrt();
        if length > f32::EPSILON { // Проверка на нулевую длину (вырожденный треугольник)
            nx /= length;
            ny /= length;
            nz /= length;
        } else {
            // Если треугольник вырожденный (точки на одной прямой), задаем дефолтную нормаль
            nx = 0.0;
            ny = 0.0;
            nz = 1.0;
        }

        // 5. Формируем структуру треугольника для stl_io
        triangles.push(stl_io::Triangle {
            normal: stl_io::Vector([nx, ny, nz]),
            vertices: [
                stl_io::Vector(v0),
                stl_io::Vector(v1),
                stl_io::Vector(v2),
            ],
        });
    }

    // 6. Запись в бинарный STL-формат в буфер памяти
    let mut binary_stl = Vec::new();
    stl_io::write_stl(&mut binary_stl, triangles.iter())
        .map_err(|err| error.pass_with("stl_io::write_stl", err.to_string()))?;

    Ok(binary_stl)
}