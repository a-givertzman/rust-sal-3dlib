use crate::{load_stl, tools::*, write_stl};
use baby_shark::{
    decimation::{ConstantErrorDecimationCriteria, EdgeDecimator}, mesh::corner_table::{CornerTableD, CornerTableF} 
};
use parry3d_f64::{math::{Pose, Vec3}, shape::{TriMesh, TriMeshFlags}};
use sal_core::dbg::Dbg;
use std::{io::prelude::Write, path::{Path, PathBuf}, sync::Arc};

#[test]
fn optimize_sofia() {
    let path = "src/tests/assets/hull.stl";
    let src_mesh = load_stl(Path::new(path), 1000.).unwrap();
    let optimized_mesh = optimize(&src_mesh, 0.1).unwrap();
    let src_aabb = src_mesh.aabb(&Pose::identity());
    let optimized_aabb = optimized_mesh.aabb(&Pose::identity());
    assert!((src_aabb.mins - optimized_aabb.mins).length() <= 0.1, "{}", format!("src:{:?} opt:{:?}", src_aabb.mins, optimized_aabb.mins));
    assert!((src_aabb.maxs - optimized_aabb.maxs).length() <= 0.1, "{}", format!("src:{:?} opt:{:?}", src_aabb.maxs, optimized_aabb.maxs));
    let src_properties = properties(&src_mesh, 1.);
    let optimized_properties = properties(&optimized_mesh, 1.);
    assert!((src_properties.0 - optimized_properties.0).abs() <= 100., "{}", format!("src:{:?} opt:{:?}", src_properties.0, optimized_properties.0));
    assert!((src_properties.1 - optimized_properties.1).len() <= 0.1, "{}", format!("src:{:?} opt:{:?}", src_properties.1, optimized_properties.1));
}
