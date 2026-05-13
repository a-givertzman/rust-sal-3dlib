use crate::{load_stl, tools::*};
use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::time::Instant;
use std::{path::Path, sync::Arc};

#[ignore]
#[test]
fn load_large_stl() {
    let path = "src/tests/assets/SURF_Sofiya.stl";
    let time = Instant::now();
    let mesh_no_scale = load_stl(Path::new(path), 1.).unwrap();
    let elapsed_no_scale = time.elapsed();
    let time = Instant::now();
    let mesh_scale = load_stl(Path::new(path), 1000.).unwrap();
    let elapsed_scale = time.elapsed();    
    println!("no_scale, time:{:?} vertices:{};\n scale, time:{:?} vertices:{}",
    elapsed_no_scale, mesh_no_scale.vertices().len(),
    elapsed_scale, mesh_scale.vertices().len(),);
}
