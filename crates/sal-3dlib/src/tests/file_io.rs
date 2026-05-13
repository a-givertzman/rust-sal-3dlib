use crate::{load_stl, tools::*};
use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::time::Instant;
use std::{path::Path, sync::Arc};


#[test]
fn load_large_stl() {
    let path = "src/tests/assets/SURF_Sofiya.stl";
    let time = Instant::now();
    let mesh = load_stl(Path::new(path), 1000.).unwrap();
    let elapsed = time.elapsed();
    println!("time:{:?} vertices:{}", elapsed, mesh.vertices().len(),);
}
