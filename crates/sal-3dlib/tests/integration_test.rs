use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_3dlib::{
    fs::Reader,
    gmath::Vector,
    ops::{
        boolean::{
            volume::{Volume, VolumeConf},
            Intersect, OpConf,
        },
        transform::{Rotate, Translate},
        Polygon,
    },
    props::{Center, Metadata, Volume as _},
    topology::{
        shape::{Face, Vertex, Wire},
        Shape,
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
        (5005709565087.305, [56935.84716518255, -48.799994830168544, 3486.7006485114216]),
        (4997123611150.073, [56901.450370019294, -43.072351524908186, 3480.6627950741686]),
        (4990354422361.824, [56904.56157345559, -34.597901718182015, 3475.753613884537]),
        (4985498817542.236, [56920.73610703446, -24.426901311041206, 3472.1471248605703]),
        (4982497563747.236, [56931.176414339825, -13.19332428964617, 3469.938243743931]),
        (4981540570377.869, [56938.70755109607, 3.724179800343274e-6, 3469.179500084623]),
        (4982536825629.566, [56931.17653219075, 13.193335656764816, 3469.9382439423443]),
        (4985500773729.179, [56920.73601403483, 24.426892105948813, 3472.147124539122]),
        (4990365420049.641, [56904.56155686395, 34.59790011122823, 3475.7536138003206]),
        (4997124435443.456, [56901.45036231205, 43.0723501768116, 3480.6627949799017]),
        (5005704647765.566, [56935.84716764531, 48.79999512200186, 3486.700648536954])
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
            .translated(Vector::new(0.0, 0.0, delta_z))
            .rotated(hull.center(), Vector::unit_x(), init_plane_rotation)
    };
    log::debug!("*plane* | elapsed: {:?}", time.elapsed());
    for (step, (volume, center)) in test_data.into_iter().enumerate() {
        println!("step {}", step);
        // the center returns value instantly
        let plane_center = plane.center();
        time = Instant::now();
        plane = plane.rotated(plane_center, Vector::unit_x(), DEG_IN_RAD);
        log::debug!("*plane*.rotated | elapsed: {:?}", time.elapsed());
        time = Instant::now();
        let bottom_part = hull.volume(&plane, vol_conf).expect("Ok(*bottom_part*)");
        log::debug!("*hull*.volumed | elapsed: {:?}", time.elapsed());
        let (result, target) = (bottom_part.volume(), volume);
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
        (5005709565087.305, [56935.84716518255, -48.799994830168544, 3486.7006485114216]),
        (4997123611150.073, [56901.450370019294, -43.072351524908186, 3480.6627950741686]),
        (4990354422361.631, [56904.56157345161, -34.59790171755873, 3475.7536138845044]),
        (4985498817542.185, [56920.736107033496, -24.426901311337474, 3472.1471248605803]),
        (4982497563746.292, [56931.1764143346, -13.193324288969734, 3469.938243743919]),
        (4981540570377.871, [56938.707551096246, 3.7241728961477914e-6, 3469.179500084623]),
        (4982536825629.186, [56931.176532188365, 13.193335656623049, 3469.9382439423416]),
        (4985500773729.133, [56920.73601403414, 24.426892105761226, 3472.1471245391153]),
        (4990365420049.668, [56904.561556863766, 34.59790011121322, 3475.753613800319]),
        (4997124435442.806, [56901.45036231188, 43.07235017788082, 3480.6627949799745]),
        (5005704647765.351, [56935.84716764663, 48.799995121953316, 3486.700648536949])
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
            .translated(Vector::new(0.0, 0.0, delta_z))
            .rotated(hull.center(), Vector::unit_x(), init_plane_rotation)
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
                    plane = plane.rotated(
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
                    let (result, target) = (bottom_part.volume(), volume);
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
                .expect("Attributes set on reading file")
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
        let (result, target) = (model.attrs().unwrap().custom(), &user_data.0);
        assert_eq!(
            result, target,
            "step {} *model*.attrs.custom\n result: {:?}\ntarget: {:?}",
            step, result, target
        );
        println!(
            "step {} *model*.attrs.custom\n result: {:?}\n target: {:?}",
            step, result, target
        );
        *model.attrs_mut().unwrap().custom_mut() = user_data.1;
    }
    test_duration.exit();
}
