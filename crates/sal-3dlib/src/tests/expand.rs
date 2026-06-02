use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::path::Path;
use crate::{io::trimesh::*, tests::local_cache::{DisplacementCache, LocalCache}, tools::*};


#[test]
fn expand_mesh() {
 //   let src_path = "src/tests/assets/Sofiya_4work.stl";
  //  let dst_path = "src/tests/assets/Sofiya_4work_expanded.stl";
  //  let src_path = "src/tests/assets/JAPAN.stl";
  //  let dst_path = "src/tests/assets/JAPAN_expanded.stl";
    let src_path = "src/tests/assets/hull.stl";
    let dst_path = "src/tests/assets/hull_expanded.stl";  
    let mesh = load(Path::new(src_path), 1.).unwrap();
    let mesh = expand_closed_ship_trimesh(&mesh, 10.);
    write(Path::new(dst_path), &mesh).unwrap();
}
