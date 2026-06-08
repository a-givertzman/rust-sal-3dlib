use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::path::Path;
use crate::{io::trimesh::*, tests::local_cache::{DisplacementCache, LocalCache}, tools::*};

#[test]
fn hydrostatic_sofia() {
    let dbg = Dbg::new("test", "hydrostatic_sofia");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;
    let mut cache = DisplacementCache::new(&dbg, "src/tests/assets/displacement_cache_hull".into());
    dbg!(cache.init().unwrap());
    /*  let heel_steps = vec![
        -60., -50., -45., -40., -35., -30., -25., -20., -15., -10., -5., -2., -1., -0.5, -0.2, 0.,
        0.2, 0.5, 1., 2., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 60.,
    ];
    let trim_steps = vec![
        -40., -30., -25., -20., -15., -12.5, -10., -7.5, -5., -3., -2., -1., -0.5, -0.2, 0., 0.2,
        0.5, 1., 2., 3., 5., 7.5, 10., 12.5, 20., 25., 30., 40.,
    ];
    let draught_steps: Vec<_> = (1..=28).map(|v| (v as f64) * 0.5).collect();*/
    let heel_steps = vec![-60., -20., -5., 0., 5., 20., 60.];
    let trim_steps = vec![-40., -10., -2., 0., 2., 10., 40.];
    let draught_steps: Vec<_> = (1..=7).map(|v| (v as f64) * 2.).collect();
    let epsilon_volume_abs = 10.;
    let epsilon_volume_percent = 0.1;
    let epsilon_center_abs = 0.01;
    let epsilon_center_percent = 0.1;
    let mut results = Vec::new();
    for &heel in &heel_steps {
        for &trim in &trim_steps {
            for &draught in &draught_steps {
                let (result_volume, result_center) =
                    calculate_hydrostatic(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
                let target = cache.get_from_level(heel, trim, draught);
                let check = |text: String,
                             result: f64,
                             target: f64,
                             epsilon_abs: f64,
                             epsilon_percent: f64|
                 -> Option<(f64, f64, String, f64, f64)> {
                    let delta_abs = (result - target).abs();
                    let delta_percent = if target > 0. {
                        delta_abs * 100. / target
                    } else {
                        0.
                    };
                    /*    println!(
                        "{text} result:{result} target:{target} delta_abs:{delta_abs} delta_percent:{delta_percent}"
                    );*/
                    if delta_abs > epsilon_abs && delta_percent > epsilon_percent {
                        return Some((delta_abs, delta_percent, text, result, target));
                    }
                    None
                };
                check(
                    format!("{:.3} {:.3} {:.1} volume", heel, trim, draught),
                    result_volume,
                    target.volume,
                    epsilon_volume_abs,
                    epsilon_volume_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.x", heel, trim, draught),
                    result_center.x,
                    target.volume_center.x(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.y", heel, trim, draught),
                    result_center.y,
                    target.volume_center.y(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.z", heel, trim, draught),
                    result_center.z,
                    target.volume_center.z(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
            }
        }
    }
    results.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    //  let _ = results.split_off(100);
    for v in results {
        println!(
            "{} error: result:{} target:{} delta_abs:{} delta_percent:{}",
            v.2, v.3, v.4, v.0, v.1
        );
    }
}

#[test]
fn hydrostatic_waterline_sofia1() {
    let dbg = Dbg::new("test", "hydrostatic_waterline_sofia1");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;
    let mut cache = DisplacementCache::new(&dbg, "src/tests/assets/displacement_cache_hull".into());
    dbg!(cache.init().unwrap());
    let heel_steps = vec![
        -60., -50., -45., -40., -35., -30., -25., -20., -15., -10., -5., -2., -1., -0.5, -0.2, 0.,
        0.2, 0.5, 1., 2., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 60.,
    ];
    let trim_steps = vec![
        -40., -30., -25., -20., -15., -12.5, -10., -7.5, -5., -3., -2., -1., -0.5, -0.2, 0., 0.2,
        0.5, 1., 2., 3., 5., 7.5, 10., 12.5, 20., 25., 30., 40.,
    ];
    let draught_steps: Vec<_> = (1..=28).map(|v| (v as f64) * 0.5).collect();
    //  let heel_steps = vec![-20., 0., 20.];
    // let trim_steps = vec![ -20., 0., 20., ];
    //   let draught_steps: Vec<_> = vec![4.];
    let epsilon_volume_abs = 10.;
    let epsilon_volume_percent = 0.1;
    let epsilon_center_abs = 0.01;
    let epsilon_center_percent = 0.1;
    let mut results = Vec::new();
    for &heel in &heel_steps {
        for &trim in &trim_steps {
            for &draught in &draught_steps {
                let (result_area, result_center) =
                    calculate_waterline(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
                /*    println!(
                    "{:.3} {:.3} {:.3}: area:{:.3} x:{:.3} y:{:.3} z:{:.3}",
                    heel,
                    trim,
                    draught,
                    result_area,
                    result_center.x,
                    result_center.y,
                    result_center.z,
                );*/
                let target = cache.get_from_level(heel, trim, draught);
                let check = |text: String,
                             result: f64,
                             target: f64,
                             epsilon_abs: f64,
                             epsilon_percent: f64|
                 -> Option<(f64, f64, String, f64, f64)> {
                    let delta_abs = (result - target).abs();
                    let delta_percent = if target > 0. {
                        delta_abs * 100. / target
                    } else {
                        0.
                    };
                    /*    println!(
                        "{text} result:{result} target:{target} delta_abs:{delta_abs} delta_percent:{delta_percent}"
                    );*/
                    if delta_abs > epsilon_abs && delta_percent > epsilon_percent {
                        return Some((delta_abs, delta_percent, text, result, target));
                    }
                    None
                };
                check(
                    format!("{:.3} {:.3} {:.1} area", heel, trim, draught),
                    result_area,
                    target.area_wl,
                    epsilon_volume_abs,
                    epsilon_volume_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.x", heel, trim, draught),
                    result_center.x,
                    target.area_wl_center.x(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.y", heel, trim, draught),
                    result_center.y,
                    target.area_wl_center.y(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} center.z", heel, trim, draught),
                    result_center.z,
                    target.area_wl_center.z(),
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
            }
        }
    }
    results.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    let _ = results.split_off(100);
    for v in results {
        println!(
            "{} error: result:{} target:{} delta_abs:{} delta_percent:{}",
            v.2, v.3, v.4, v.0, v.1
        );
    }
}

#[test]
fn hydrostatic_waterline_sofia2() {
    let dbg = Dbg::new("test", "hydrostatic_waterline_sofia2");
    let path = "src/tests/assets/Sofiya_4work.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;
    /*  let heel_steps = vec![
        -60., -50., -45., -40., -35., -30., -25., -20., -15., -10., -5., -2., -1., -0.5, -0.2, 0.,
        0.2, 0.5, 1., 2., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 60.,
    ];
    let trim_steps = vec![
        -40., -30., -25., -20., -15., -12.5, -10., -7.5, -5., -3., -2., -1., -0.5, -0.2, 0., 0.2,
        0.5, 1., 2., 3., 5., 7.5, 10., 12.5, 20., 25., 30., 40.,
    ];
    let draught_steps: Vec<_> = (1..=28).map(|v| (v as f64) * 0.5).collect();*/
    let heel_steps = vec![
        -20., 0., 20.,
        //        0.,
    ];
    let trim_steps = vec![
        -20., 0., 20.,
        //         0.,
    ];
    let draught_steps: Vec<_> = (4..=10).map(|v| v as f64).collect();
    for &heel in &heel_steps {
        for &trim in &trim_steps {
            for &draught in &draught_steps {
                let (area, center) = calculate_waterline(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
                println!(
                    "{:.3} {:.3} {:.3}: area:{:.3} x:{:.3} y:{:.3} z:{:.3}",
                    heel, trim, draught, area, center.x, center.y, center.z,
                );
            }
        }
    }
}

#[test]
fn hydrostatic_inertia_sofia1() {
    let dbg = Dbg::new("test", "hydrostatic_inertia_sofia1");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;
    let mut cache = DisplacementCache::new(&dbg, "src/tests/assets/displacement_cache_hull".into());
    dbg!(cache.init().unwrap());
    let heel_steps = vec![
        -60., -50., -45., -40., -35., -30., -25., -20., -15., -10., -5., -2., -1., -0.5, -0.2, 0.,
        0.2, 0.5, 1., 2., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 60.,
    ];
    let trim_steps = vec![
        -40., -30., -25., -20., -15., -12.5, -10., -7.5, -5., -3., -2., -1., -0.5, -0.2, 0., 0.2,
        0.5, 1., 2., 3., 5., 7.5, 10., 12.5, 20., 25., 30., 40.,
    ];
    let draught_steps: Vec<_> = (1..=28).map(|v| (v as f64) * 0.5).collect();
    //  let heel_steps = vec![-20., 0., 20.];
    // let trim_steps = vec![ -20., 0., 20., ];
    //   let draught_steps: Vec<_> = vec![4.];
    let epsilon_volume_abs = 10.;
    let epsilon_volume_percent = 0.1;
    let epsilon_center_abs = 0.01;
    let epsilon_center_percent = 0.1;
    let mut results = Vec::new();
    for &heel in &heel_steps {
        for &trim in &trim_steps {
            for &draught in &draught_steps {
                let (ix, iy) = calculate_inertia(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);

                /*    println!(
                    "{:.3} {:.3} {:.3}: area:{:.3} x:{:.3} y:{:.3} z:{:.3}",
                    heel,
                    trim,
                    draught,
                    result_area,
                    result_center.x,
                    result_center.y,
                    result_center.z,
                );*/
                let target = cache.get_from_level(heel, trim, draught);
                let check = |text: String,
                             result: f64,
                             target: f64,
                             epsilon_abs: f64,
                             epsilon_percent: f64|
                 -> Option<(f64, f64, String, f64, f64)> {
                    let delta_abs = (result - target).abs();
                    let delta_percent = if target > 0. {
                        delta_abs * 100. / target
                    } else {
                        0.
                    };
                    /*    println!(
                        "{text} result:{result} target:{target} delta_abs:{delta_abs} delta_percent:{delta_percent}"
                    );*/
                    if delta_abs > epsilon_abs && delta_percent > epsilon_percent {
                        return Some((delta_abs, delta_percent, text, result, target));
                    }
                    None
                };
                check(
                    format!("{:.3} {:.3} {:.1} ix", heel, trim, draught),
                    ix,
                    target.inertia_trans_x,
                    epsilon_volume_abs,
                    epsilon_volume_percent,
                )
                .map(|v| results.push(v));
                check(
                    format!("{:.3} {:.3} {:.1} iy", heel, trim, draught),
                    iy,
                    target.inertia_long_y,
                    epsilon_center_abs,
                    epsilon_center_percent,
                )
                .map(|v| results.push(v));
            }
        }
    }
    results.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    let _ = results.split_off(100);
    for v in results {
        println!(
            "{} error: result:{} target:{} delta_abs:{} delta_percent:{}",
            v.2, v.3, v.4, v.0, v.1
        );
    }
}

#[test]
fn hydrostatic_inertia_sofia2() {
    let dbg = Dbg::new("test", "hydrostatic_inertia_sofia2");
    let path = "src/tests/assets/Sofiya_4work.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;
    let heel_steps = vec![
        -20., 0., 20.,
        //        0.,
    ];
    let trim_steps = vec![
        -20., 0., 20.,
        //         0.,
    ];
    let draught_steps: Vec<_> = vec![4.];
    for &heel in &heel_steps {
        for &trim in &trim_steps {
            for &draught in &draught_steps {
                let (ix, iy) = calculate_inertia(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
                println!(
                    "{:.3} {:.3} {:.3}: ix:{:.3} iy:{:.3}",
                    heel, trim, draught, ix, iy,
                );
            }
        }
    }
}

#[test]
fn hydrostatic_waterline_size_sofia1() {
    let dbg = Dbg::new("test", "hydrostatic_waterline_size_sofia1");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let mut cache = DisplacementCache::new(&dbg, "src/tests/assets/displacement_cache_hull".into());
    dbg!(cache.init().unwrap());
    let draught_steps: Vec<_> = (1..=28).map(|v| (v as f64) * 0.5).collect();
    let mut results = Vec::new();
    for &draught in &draught_steps {
        let (dx, dy) = calculate_waterline_size(&mesh, draught);

        //   println!( "{:.3}: dx:{:.3} dy:{:.3} ", draught, dx, dy,);
        let target = cache.get_from_level(0., 0., draught);
        let check = |text: String,
                     result: f64,
                     target: f64|
         -> (f64, f64, String, f64, f64) {
            let delta_abs = (result - target).abs();
            let delta_percent = if target > 0. {
                delta_abs * 100. / target
            } else {
                0.
            };
            /*    println!(
                "{text} result:{result} target:{target} delta_abs:{delta_abs} delta_percent:{delta_percent}"
            );*/
            return (delta_abs, delta_percent, text, result, target);
        };
        results.push(check(
            format!("{:.1} dx", draught),
            dx,
            target.length_wl,
        ));
        results.push(check(
            format!("{:.1} dy", draught),
            dy,
            target.breadth_wl,
        ));
    }
    results.sort_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());
    for v in results {
        println!(
            "{} error: result:{} target:{} delta_abs:{} delta_percent:{}",
            v.2, v.3, v.4, v.0, v.1
        );
    }
}

#[test]
fn hydrostatic_waterline_size_sofia2() {
    let dbg = Dbg::new("test", "hydrostatic_waterline_size_sofia2");
    let path = "src/tests/assets/Sofiya_4work.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let draught_steps: Vec<_> = (1..=12).map(|v| v as f64).collect();

    for &draught in &draught_steps {
        let (dx, dy) = calculate_waterline_size(&mesh, draught);
        println!("{:.3}: dx:{:.3} dy:{:.3}", draught, dx, dy,);
    }
}

#[test]
fn hydrostatic_cross_sections_sofia() {
    let dbg = Dbg::new("test", "hydrostatic_cross_sections_sofia");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;

    // Фиксируем посадку судна для проверки строевой по шпангоутам
    let heel = 0.0;
    let trim = 0.0;
    let draught = 5.0; // Осадка 5 метров

    let isometry = position(&Vec3::new(dx, 0., 0.), heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);
    let sliced_mesh = plane.slice_mesh(&mesh);

    // Генерируем координаты шпангоутов вдоль оси X (например, от 0 до 140 метров с шагом 2 метра)
    let step_size = 0.1;
    let mut x = -3.6;
    let x_max = 135.5;
    let mut x_steps: Vec<_> = vec![];
    while x <= x_max {
        x_steps.push(x - dx);
        x += step_size;
    }

    println!("\n--- Расчет теоретических шпангоутов (Draught: {}, Heel: {}, Trim: {}) ---", draught, heel, trim);
    println!("X_coord\t\tArea (м²)\tWidth_B (м)\tHeight_T (м)");

    let mut integrated_volume = 0.0;
    let mut last_area = 0.0;

    for (i, &x) in x_steps.iter().enumerate() {
        // Вызываем добавленную функцию верхнего уровня
        let (area, width, height) = calculate_cross_section_at(&sliced_mesh, isometry, x);
        
        if i % 100 == 0 {
            println!("{:<12.2}\t{:<12.3}\t{:<12.3}\t{:<12.3}", x, area, width, height);
        }

        // Интегрируем площади шпангоутов по длине методом трапеций (Метод Кавальери)
        if i > 0 {
            integrated_volume += (last_area + area) * 0.5 * step_size;
        }
        last_area = area;
    }

    // Сверяем полученный объем с эталонным расчетом через тетраэдры Гаусса
    let (mesh_volume, _) = calculate_hydrostatic(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
    
    println!("\n--- Верификация геометрии шпангоутов ---");
    println!("Объем через 3D тетраэдры (Гаусс):       {:.3} м³", mesh_volume);
    println!("Объем через 1D интеграл шпангоутов:     {:.3} м³", integrated_volume);
    
    let delta_percent = ((mesh_volume - integrated_volume).abs() * 100.0) / mesh_volume;
    println!("Погрешность дискретизации:              {:.3}%", delta_percent);

    // Погрешность для гладкого корпуса при шаге 2м не должна превышать 0.5-1%
    assert!(delta_percent < 1.0, "Интеграл площадей шпангоутов разошелся с объемом сетки!");
}
//
#[test]
fn hydrostatic_buttocks_sofia() {
    let dbg = Dbg::new("test", "hydrostatic_buttocks_sofia");
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let dx = 65.25;

    // Фиксируем ту же посадку судна для проверки строевой по батоксам
    let heel = 0.0;
    let trim = 0.0;
    let draught = 5.0; // Осадка 5 метров

    let isometry = position(&Vec3::new(dx, 0., 0.), heel, trim, draught).inverse();
    let local_point = isometry.transform_point(Vec3::ZERO); 
    let local_normal = isometry.transform_vector(Vec3::Z).normalize(); 
    let plane = Plane::from_point_and_normal(local_point, local_normal);
    let sliced_mesh = plane.slice_mesh(&mesh);

    // Генерируем координаты батоксов вдоль оси Y (от левого борта до правого)
    // Шаг 0.05 м обеспечит идеальную точность трапеций для скругленной скулы
    let step_size = 0.01;
    let mut y = -8.;
    let y_max = 8.;
    let mut y_steps: Vec<_> = vec![];
    while y <= y_max {
        y_steps.push(y);
        y += step_size;
    }

    println!("\n--- Расчет теоретических батоксов (Draught: {}, Heel: {}, Trim: {}) ---", draught, heel, trim);
    println!("Y_coord\t\tArea (м²)\tLength_L (м)\tHeight_T (м)");

    let mut integrated_volume = 0.0;
    let mut last_area = 0.0;

    for (i, &y) in y_steps.iter().enumerate() {
        // Вызываем функцию расчета продольного сечения
        let (area, length, height) = calculate_buttock_at(&sliced_mesh, isometry, y);
        
        // Выводим каждый 20-й батокс (шаг 1 метр) и диаметральную плоскость (Y = 0)
        if i % 100 == 0 || y.abs() < 0.01 {
            println!("{:<12.2}\t{:<12.3}\t{:<12.3}\t{:<12.3}", y, area, length, height);
        }

        // Интегрируем площади батоксов по ширине методом трапеций
        if i > 0 {
            integrated_volume += (last_area + area) * 0.5 * step_size;
        }
        last_area = area;
    }

    // Сверяем полученный объем с эталонным расчетом через тетраэдры Гаусса
    let (mesh_volume, _) = calculate_hydrostatic(&mesh, Vec3::new(dx, 0., 0.), heel, trim, draught);
    
    println!("\n--- Верификация геометрии батоксов ---");
    println!("Объем через 3D тетраэдры (Гаусс):       {:.3} м³", mesh_volume);
    println!("Объем через 1D интеграл батоксов:       {:.3} м³", integrated_volume);
    
    let delta_percent = ((mesh_volume - integrated_volume).abs() * 100.0) / mesh_volume;
    println!("Погрешность дискретизации:              {:.3}%", delta_percent);

    // Проверяем схождение методов
    assert!(delta_percent < 1.0, "Интеграл площадей батоксов разошелся с объемом сетки!");
}