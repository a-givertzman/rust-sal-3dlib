use crate::fs::Reader;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use std::{
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
/// Testing import from STEP files.
#[test]
fn import_from_step_file() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    init_each();
    let dbg_id = "import_from_step_file";
    log::debug!("\n{}", dbg_id);
    let test_duration = TestDuration::new(dbg_id, Duration::from_secs(300));
    test_duration.run().unwrap();
    let test_data = [
        (
            "src/tests/fs_test/resources/hull_shell.step",
            vec!["/my_shell"],
        ),
        (
            "src/tests/fs_test/resources/primitives.stp",
            vec!["/cube", "/shell", "/face", "/wire", "/1", "/face"],
        ),
    ];
    let mut time = Instant::now();
    for (step, (filename, model_names)) in test_data.into_iter().enumerate() {
        let imported = Reader::read_step(filename)
            .unwrap()
            .into_vec::<()>()
            .unwrap();
        let (result, target) = (imported.len(), model_names.len());
        println!("step: {}, result: {}, target: {}\n", step, result, target);
        log::debug!("result: {:?}, target: {:?}", result, target);
        assert_eq!(
            result, target,
            "step {} \nresult: {}\ntarget: {}",
            step, result, target
        );
        for (result, target) in imported.into_iter().map(|elmnt| elmnt.0).zip(model_names) {
            println!("step: {}, result: {}, target: {}\n", step, result, target);
            log::debug!("result: {:?}, target: {:?}", result, target);
            assert_eq!(
                result, target,
                "step {} \nresult: {}\ntarget: {}",
                step, result, target
            );
        }
        log::debug!("{} | elapsed: {:?}", dbg_id, time.elapsed());
        time = Instant::now();
    }
    test_duration.exit();
}
