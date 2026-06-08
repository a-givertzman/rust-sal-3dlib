use sal_core::error::Error;
use truck_meshalgo::prelude::*;
use truck_polymesh::PolygonMesh;
use truck_stepio::r#in::ruststep;

/// Точность тесселяции для преобразования математических поверхностей (NURBS) в полигоны.
const TESSELLATION_TOLERANCE: f64 = 0.01;

/// Парсинг из массива бинарных данных в формате STEP.
/// Возвращает (вектор вершин f32, вектор индексов треугольников usize).
pub fn parse(data: &[u8]) -> Result<(Vec<[f32; 3]>, Vec<[usize; 3]>), Error> {
    let error = Error::new("io.step", "parse");

    // 1. Десериализуем сетевой буфер байт в строку &str (Zero-copy)
    let step_str = std::str::from_utf8(data)
        .map_err(|err| error.pass_with("std::str::from_utf8", err.to_string()))?;

    // 2. Парсим структуру Exchange Structure файла STEP через нативный парсер ruststep
    let exchange = ruststep::parser::parse(step_str)
        .map_err(|err| error.pass_with("ruststep::parser::parse", err.to_string()))?;

    // 3. Получаем доступ к секции данных (DATA)
    let data_section = exchange.data.first()
        .ok_or_else(|| error.err("STEP file does not contain a valid DATA section"))?;

    // 4. Строим внутреннюю индексированную таблицу объектов truck_stepio
    let table = truck_stepio::r#in::Table::from_data_section(data_section);

    let mut global_vertices = Vec::new();
    let mut global_indices = Vec::new();
    let mut vertex_offset = 0;

    // 5. Итерируемся по всем изолированным оболочкам (shells), объявленным в STEP
    for (_idx, shell) in table.shell.iter() {
        // Восстанавливаем геометрию и топологию конкретной оболочки из таблицы
        let compressed_shell = table.to_compressed_shell(shell)
            .map_err(|err| error.pass_with("table.to_compressed_shell", err.to_string()))?;

        // 6. Выполняем тесселяцию (триангуляцию) поверхностей в PolygonMesh через метод .to_polygon()
        let mesh: PolygonMesh = compressed_shell.triangulation(TESSELLATION_TOLERANCE).to_polygon();

        // 7. Конвертируем координаты f64 из ядра truck во фреймворк f32
        let vertices: Vec<[f32; 3]> = mesh
            .positions()
            .iter()
            .map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();

        global_vertices.extend(vertices);

        // 8. Разложение граней полигонов на треугольники методом веера (Fan Triangulation)
        // Структура Faces итерируется через явный вызов метода .iter()
        for face in mesh.faces().face_iter() {
            if face.len() < 3 { continue; }
            
            let base_idx = face[0].pos + vertex_offset;
            for i in 1..face.len() - 1 {
                global_indices.push([
                    base_idx,
                    face[i].pos + vertex_offset,
                    face[i + 1].pos + vertex_offset,
                ]);
            }
        }
        
        vertex_offset = global_vertices.len();
    }

    if global_vertices.is_empty() {
        return Err(error.err("STEP file parsed successfully but yielded 0 vertices/shells"));
    }

    Ok((global_vertices, global_indices))
}
