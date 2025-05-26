use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_3dlib::{
    fs::Reader,
    gmath::vector::Vector,
    ops::{
        boolean::{
            volume::{Volume, VolumeConf},
            Intersect, OpConf,
        },
        transform::{Rotate, Translate},
        Polygon,
    },
    props::{Center, Metadata, Volume as VolumeProp},
    topology::{
        shape::Shape,
        shape::{face::Face, vertex::Vertex, wire::Wire},
    },
};
use std::{
    f64::consts::PI,
    sync::Once,
    time::{Duration, Instant},
};
use testing::stuff::max_test_duration::TestDuration;
//
//
static INIT: Once = Once::new();
///
/// Once called initialisation.
fn init_once() {
    //
    // Implement your initialisation code to be called only once for current test file.
    INIT.call_once(|| {})
}
///
/// Returns:
///  - ...
#[allow(clippy::unused_unit)]
fn init_each() -> () {}
///
/// Расчеты сечения корпуса `ватерлинией`. Будем
/// - крутить и двигать корпус по вертикали,
/// - будем сечь его горизонтальной линией.
///
/// Надо будет найти:
/// - получившийся объем,
/// - координаты центральной точки по горизонтали.
#[test]
fn calc_volume_and_center_of_hull() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg_id = "calc_volume_and_center_of_hull";
    log::debug!("\n{}", dbg_id);
    let test_duration = TestDuration::new(dbg_id, Duration::from_secs(300));
    test_duration.run().unwrap();
    const DEG_IN_RAD: f64 = PI / 180.0;
    // configuration
    let init_plane_rotation = -6.0 * DEG_IN_RAD;
    let vol_conf = VolumeConf { parallel: true };
    let op_conf = OpConf { parallel: true };
    // Each row represents the volume a bottom part and its top plane center.
    // The part is built used the volume operation to the hull and the rotated plane.
    #[rustfmt::skip]
    let test_data = [
       (4996664766374.211, [56937.93523450112, -49.04428597968115, 3480.8266830543953]),
       (4989778107355.2295, [56903.13228581339, -43.25678861885399, 3475.8847377461493]),
       (4983067698868.696, [56911.1095487788, -34.554880089673986, 3471.0038999403546]),
       (4976649169431.472, [56928.92679007335, -24.374832832987632, 3466.383873142822]),
       (4970528938967.269, [56943.37477827544, -13.136938242860253, 3462.1061806220478]),
       (4964738607794.354, [56958.1567800256, 0.0000046626337154326395, 3458.2232908211704]),
       (4959280591056.395, [56954.377647474044, 13.082132095334886, 3454.798056595404]),
       (4954236244267.303, [56949.03383278765, 24.22305393928127, 3451.7597841012803]),
       (4949427831508.84, [56941.194643878385, 34.35533843650766, 3449.054113224853]),
       (4944805695116.0, [56929.97790497446, 43.626209653169376, 3446.637294977448]),
       (4940498885728.324, [56950.41248217174, 50.50253898026267, 3444.3291172619406]),
    ];
    let mut time = Instant::now();
    let tree = Reader::read_step("tests/hull_shell.step")
        .expect("read STEP file")
        .into_vec::<()>()
        .expect("turn into *tree*");
    // let tree = read_step::<()>("tests/hull_shell.step").expect("read STEP file");
    log::debug!("*tree* | elapsed: {:?}", time.elapsed());
    let (result, target) = (tree.len(), 1);
    assert_eq!(result, target, "\nresult: {}\ntarget: {}", result, target);
    println!("*tree*.len \n result: {}\n target: {}", result, target);
    let hull = match tree
        .into_iter()
        .next()
        .expect("Imported STEP file has only one model in the root")
    {
        (_, Shape::Shell(s)) => s,
        _ => panic!("Imported model is not Shell"),
    };
    time = Instant::now();
    // Build a plane, which crosses _hull_ above the middle
    // and has length capacity at each side.
    let mut plane = {
        let hull_center = hull.center().point();
        let delta_x = 70_000.0;
        let delta_y = 10_000.0;
        let delta_z = 2000.0;
        let polygon = Wire::polygon(
            [
                Vertex::new([
                    hull_center[0] + delta_x,
                    hull_center[1] + delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] - delta_x,
                    hull_center[1] + delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] - delta_x,
                    hull_center[1] - delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] + delta_x,
                    hull_center[1] - delta_y,
                    hull_center[2],
                ]),
            ],
            true,
        )
        .expect("built *polygon*");
        // Create Face from the polygon,
        // push it up along OZ axis
        // and rotate on 6 degree around OX.
        Face::try_from(&polygon)
            .expect("created *plane*")
            .translate(Vector::new(0.0, 0.0, delta_z))
            .rotate(hull.center(), Vector::unit_x(), init_plane_rotation)
    };
    log::debug!("*plane* | elapsed: {:?}", time.elapsed());
    for (step, (volume, center)) in test_data.into_iter().enumerate() {
        println!("step {}", step);
        // the center returns value instantly
        let plane_center = plane.center();
        time = Instant::now();
        plane = plane.rotate(plane_center, Vector::unit_x(), DEG_IN_RAD);
        log::debug!("*plane*.rotated | elapsed: {:?}", time.elapsed());
        time = Instant::now();
        let bottom_part = hull.volume(&plane, vol_conf).expect("Ok(*bottom_part*)");
        log::debug!("*hull*.volumed | elapsed: {:?}", time.elapsed());
        let (result, target) = (VolumeProp::volume(&bottom_part), volume);
        assert!(
            (result - target).abs() < f64::EPSILON,
            "step {} *volume*\n result: {}\n target: {}",
            step,
            result,
            target
        );
        println!(" *volume*\n result: {}\n target: {}", result, target);
        time = Instant::now();
        let top_plane = bottom_part.intersect(&plane, op_conf);
        log::debug!("*bottom_part*.intersected | elapsed: {:?}", time.elapsed());
        let (result, target) = (top_plane.center().point(), center);
        for ((result, target), coord) in result.into_iter().zip(target).zip('x'..='z') {
            assert!(
                (result - target).abs() < f64::EPSILON,
                "step {} *center*.{}\n result: {}\n target: {}",
                step,
                coord,
                result,
                target
            );
        }
        println!(" *center*\n result: {:?}\n target: {:?}", result, target);
    }
    test_duration.exit();
}
///
/// Задача аналогичная [calc_volume_and_center_of_hull],
/// но вычисления произовдятся параллельно.
#[test]
fn calc_volume_and_center_of_hull_parallel() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg_id = "calc_volume_and_center_of_hull_parallel";
    log::debug!("\n{}", dbg_id);
    let test_duration = TestDuration::new(dbg_id, Duration::from_secs(300));
    test_duration.run().unwrap();
    const DEG_IN_RAD: f64 = PI / 180.0;
    // configuration
    let init_plane_rotation = -6.0 * DEG_IN_RAD;
    let vol_conf = VolumeConf { parallel: true };
    let op_conf = OpConf { parallel: true };
    // Each row represents the volume a bottom part and its top plane center.
    // The part is built used the volume operation to the hull and the rotated plane.
    #[rustfmt::skip]
    let test_data = [
        (4996664766374.211, [56937.93523450112, -49.04428597968115, 3480.8266830543953]),
        (4989778107355.2295, [56903.13228581339, -43.25678861885399, 3475.884737746149]),
        (4983067698868.709, [56911.10954877941, -34.55488008959486, 3471.00389994035]),
        (4976649169431.35, [56928.92679007462, -24.37483283296945, 3466.383873142822]),
        (4970528938967.407, [56943.37477827667, -13.136938243048519, 3462.106180622051]),
        (4964738607794.38, [56958.15678002573, 0.000004662596248001609, 3458.2232908211704]),
        (4959280591054.918, [56954.37764746138, 13.082132094303006, 3454.7980565953844]),
        (4954236244267.209, [56949.033832786634, 24.22305393928644, 3451.75978410128]),
        (4949427831508.803, [56941.1946438788, 34.355338436628, 3449.0541132248577]),
        (4944805695115.91, [56929.977904974614, 43.626209653189605, 3446.637294977448]),
        (4940498885728.366, [56950.412482171945, 50.50253898019696, 3444.329117261934]),
    ];
    let mut time = Instant::now();
    let tree = Reader::read_step("tests/hull_shell.step")
        .expect("read STEP file")
        .into_vec::<()>()
        .expect("turn into *tree*");
    log::debug!("*tree* | elapsed: {:?}", time.elapsed());
    let (result, target) = (tree.len(), 1);
    assert_eq!(result, target, "\nresult: {}\ntarget: {}", result, target);
    println!("*tree*.len \n result: {}\n target: {}", result, target);
    let hull = match tree
        .into_iter()
        .next()
        .expect("Imported STEP file has only one model in the root")
    {
        (_, Shape::Shell(s)) => s,
        _ => panic!("Imported model is not Shell"),
    };
    time = Instant::now();
    // Build a plane, which crosses _hull_ above the middle
    // and has length capacity at each side.
    let plane = {
        let hull_center = hull.center().point();
        let delta_x = 70_000.0;
        let delta_y = 10_000.0;
        let delta_z = 2000.0;
        let polygon = Wire::polygon(
            [
                Vertex::new([
                    hull_center[0] + delta_x,
                    hull_center[1] + delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] - delta_x,
                    hull_center[1] + delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] - delta_x,
                    hull_center[1] - delta_y,
                    hull_center[2],
                ]),
                Vertex::new([
                    hull_center[0] + delta_x,
                    hull_center[1] - delta_y,
                    hull_center[2],
                ]),
            ],
            true,
        )
        .expect("built *polygon*");
        // Create Face from the polygon,
        // push it up along OZ axis
        // and rotate on 6 degree around OX.
        Face::try_from(&polygon)
            .expect("created *plane*")
            .translate(Vector::new(0.0, 0.0, delta_z))
            .rotate(hull.center(), Vector::unit_x(), init_plane_rotation)
    };
    log::debug!("*plane* | elapsed: {:?}", time.elapsed());
    std::thread::scope(|scp| {
        for (step, ((volume, center), deg)) in test_data.into_iter().zip(1..).enumerate() {
            scp.spawn({
                let hull = hull.clone();
                let center_point = plane.center().point();
                let mut plane = plane.clone();
                move || {
                    let mut time = Instant::now();
                    plane = plane.rotate(
                        Vertex::new(center_point),
                        Vector::unit_x(),
                        deg as f64 * DEG_IN_RAD,
                    );
                    log::debug!(
                        "step {} *plane*.rotated | elapsed: {:?}",
                        step,
                        time.elapsed()
                    );
                    time = Instant::now();
                    let bottom_part = hull.volume(&plane, vol_conf).expect("Ok(*bottom_part*)");
                    log::debug!(
                        "step {} *hull*.volumed | elapsed: {:?}",
                        step,
                        time.elapsed()
                    );
                    let (result, target) = (VolumeProp::volume(&bottom_part), volume);
                    assert!(
                        (result - target).abs() < f64::EPSILON,
                        "step {} *volume*\n result: {}\n target: {}",
                        step,
                        result,
                        target
                    );
                    println!(
                        "step {} *volume*\n result: {}\n target: {}",
                        step, result, target
                    );
                    time = Instant::now();
                    let top_plane = bottom_part.intersect(&plane, op_conf);
                    log::debug!(
                        "step {} *bottom_part*.intersected | elapsed: {:?}",
                        step,
                        time.elapsed()
                    );
                    let (result, target) = (top_plane.center().point(), center);
                    for ((result, target), coord) in result.into_iter().zip(target).zip('x'..='z') {
                        assert!(
                            (result - target).abs() < f64::EPSILON,
                            "step {} *center*.{}\n result: {}\n target: {}",
                            step,
                            coord,
                            result,
                            target
                        );
                    }
                    println!(
                        "step {} *center*\n result: {:?}\n target: {:?}",
                        step, result, target
                    );
                }
            });
        }
    });
    test_duration.exit();
}
///
/// Read from and write to the user-defined attribute.
#[test]
fn rw_user_defined_attribute() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg_id = "rw_user_defined_attribute";
    log::debug!("\n{}", dbg_id);
    let test_duration = TestDuration::new(dbg_id, Duration::from_secs(300));
    test_duration.run().unwrap();
    type TestData = Vec<u8>;
    let test_data = [
        ("/my_shell", (None, Some("test_data".as_bytes().to_vec()))),
        ("/my_shell", (Some("test_data".as_bytes().to_vec()), None)),
    ];
    let time = Instant::now();
    let tree = Reader::read_step("tests/hull_shell.step")
        .expect("read STEP file")
        .into_vec::<TestData>()
        .expect("turn into *tree*");
    // let tree = read_step::<TestData>("tests/hull_shell.step").expect("read STEP file");
    log::debug!("*tree* | elapsed: {:?}", time.elapsed());
    let (result, target) = (tree.len(), 1);
    assert_eq!(result, target, "\nresult: {}\ntarget: {}", result, target);
    println!("*tree*.len \n result: {}\n target: {}", result, target);
    let mut model = match tree
        .into_iter()
        .next()
        .expect("Imported STEP file has only one model in the root")
    {
        (_, Shape::Shell(s)) => s,
        _ => panic!("Imported model is not Shell"),
    };
    for (step, (name, user_data)) in test_data.into_iter().enumerate() {
        let (result, target) = (
            model
                .attrs()
                .name(),
            name,
        );
        assert_eq!(
            result, target,
            "step {} *model*.attrs.name\n result: {}\ntarget: {}",
            step, result, target
        );
        println!(
            "step {} *model*.attrs.name\n result: {}\n target: {}",
            step, result, target
        );
        let (result, target) = (model.attrs().custom(), &user_data.0);
        assert_eq!(
            result, target,
            "step {} *model*.attrs.custom\n result: {:?}\ntarget: {:?}",
            step, result, target
        );
        println!(
            "step {} *model*.attrs.custom\n result: {:?}\n target: {:?}",
            step, result, target
        );
        *model.attrs_mut().custom_mut() = user_data.1;
    }
    test_duration.exit();
}
