use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::path::Path;
use crate::{load_stl, tests::local_cache::{DisplacementCache, LocalCache}, tools::*, write_stl};


#[test]
fn expand_mesh() {
 //   let src_path = "src/tests/assets/Sofiya_4work.stl";
  //  let dst_path = "src/tests/assets/Sofiya_4work_expanded.stl";
  //  let src_path = "src/tests/assets/JAPAN.stl";
  //  let dst_path = "src/tests/assets/JAPAN_expanded.stl";
    let src_path = "src/tests/assets/hull.stl";
    let dst_path = "src/tests/assets/hull_expanded.stl";  
    let mesh = load_stl(Path::new(src_path), 1.).unwrap();
    let mesh = expand_closed_ship_trimesh(&mesh, 10.);
    write_stl(Path::new(dst_path), &mesh).unwrap();
}
