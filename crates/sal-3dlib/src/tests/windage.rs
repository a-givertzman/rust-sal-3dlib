use crate::{io::trimesh::*, tools::*};
use parry3d_f64::math::Vec3;
use sal_core::dbg::Dbg;
use std::{path::Path, sync::Arc};


#[test]
fn windage_interval_sofia() {
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let target = [
        (5., 1236.268, 60.460, 10.873),
        (6., 1105.868, 59.704, 11.507),
        (7., 973.025, 59.112, 12.190),
        (8., 839.143, 58.369, 12.939),
        (9., 704.458, 57.277, 13.787),
    ];

    let midel_dx = 65.25;
    let lbp = 130.5;
    let draught_min = 2.001;
    let windage = WindageProfile::new(Arc::new(mesh), midel_dx, draught_min, lbp, 10000);
    for (draught, target_area, target_sx, target_sz) in &target {
        let (area, _) = windage.calculate_area(*draught, 0.);
        let res_sx = area.center_x;
        let res_sz = area.center_z; 
        let res_area_array: f64 = windage.calculate_area_array(*draught, 0.).iter().sum();
        println!(
            "draught:{:.3} area:({:.3} {:.3} {:.3}): sx:({:.3} {:.3} {:.3}) sz:({:.3} {:.3} {:.3})",
            draught, 
            target_area, area.area, res_area_array, 
            target_sx, res_sx, target_sx - res_sx,
            target_sz, res_sz, target_sz - res_sz,
        );
    }
}

/*
#[test]
fn windage_triangles_sofia() {
    let scale = 0.001f64;
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path)).scaled(Vec3::new(scale, scale, scale));
    let target = [
        (5., 1236.268, 60.460, 10.873),
        (6., 1105.868, 59.704, 11.507),
        (7., 973.025, 59.112, 12.190),
        (8., 839.143, 58.369, 12.939),
        (9., 704.458, 57.277, 13.787),
    ];

    let midel_dx = 65.25;
    let windage = windage_triangles::WindageProfile::new(&mesh, midel_dx);
    for (draught, target_area, target_sx, target_sz) in &target {
        let (res_area, mx, mz, _) = windage.calculate_area(*draught, 0.);
        let res_sx = mx/res_area;
        let res_sz = mz/res_area; 
        println!(
            "draught:{:.3} area:({:.3} {:.3} {:.3}): sx:({:.3} {:.3} {:.3}) sz:({:.3} {:.3} {:.3})",
            draught, 
            target_area, res_area, target_area - res_area, 
            target_sx, res_sx, target_sx - res_sx,
            target_sz, res_sz, target_sz - res_sz,
        );
    }
}

#[test]
fn windage_voxels_sofia() {
    let scale = 0.001f64;
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path)).scaled(Vec3::new(scale, scale, scale));
    let target = [
        (5., 1236.268, 60.460, 10.873),
        (6., 1105.868, 59.704, 11.507),
        (7., 973.025, 59.112, 12.190),
        (8., 839.143, 58.369, 12.939),
        (9., 704.458, 57.277, 13.787),
    ];
    let lbp = 130.5;
    let draught_min = 2.001;
    let midel_from_stern = 68.82;
    let windage = windage_voxels::WindageProfile::new(&mesh, midel_from_stern, draught_min, lbp, 10000);
    for (draught, target_area, target_sx, target_sz) in &target {
        let (res_area, mx, mz, _) = windage.calculate_area(*draught, 0.).unwrap();
        let res_sx = mx/res_area;
        let res_sz = mz/res_area; 
        println!(
            "draught:{:.3} area:({:.3} {:.3} {:.3}): sx:({:.3} {:.3} {:.3}) sz:({:.3} {:.3} {:.3})",
            draught, 
            target_area, res_area, target_area - res_area, 
            target_sx, res_sx, target_sx - res_sx,
            target_sz, res_sz, target_sz - res_sz,
        );
    }
}

#[test]
fn windage_edge_sofia() {
    let scale = 0.001f64;
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path)).scaled(Vec3::new(scale, scale, scale));
    let target = [
        (5., 1236.268, 60.460, 10.873),
        (6., 1105.868, 59.704, 11.507),
        (7., 973.025, 59.112, 12.190),
        (8., 839.143, 58.369, 12.939),
        (9., 704.458, 57.277, 13.787),
    ];
    let lbp = 130.5;
    let draught_min = 2.001;
    let midel_from_stern = 68.82;
    let windage = windage_edge::WindageProfile::new(&mesh, midel_from_stern, draught_min, lbp);
    for (draught, target_area, target_sx, target_sz) in &target {
        let (res_area, mx, mz, _) = windage.calculate_area(*draught, 0.).unwrap();
        let res_sx = mx/res_area;
        let res_sz = mz/res_area; 
        println!(
            "draught:{:.3} area:({:.3} {:.3} {:.3}): sx:({:.3} {:.3} {:.3}) sz:({:.3} {:.3} {:.3})",
            draught, 
            target_area, res_area, target_area - res_area, 
            target_sx, res_sx, target_sx - res_sx,
            target_sz, res_sz, target_sz - res_sz,
        );
    }
}
*/

#[test]
fn windage_sofia2() {
    let dbg = Dbg::new("test", "windage_sofia2");
    //   let path = "src/tests/assets/Sofiya_4work.stl";
    let path = "src/tests/assets/hull.stl";
    let mesh = load(Path::new(path), 1000.).unwrap();
    let midel_dx = 65.25;
    let lbp = 130.5;
    let draught_min = 2.001;
    let windage = WindageProfile::new(Arc::new(mesh), midel_dx, draught_min, lbp, 10000);
    let draught_steps: Vec<_> = (2..=18).map(|v| (v as f64) * 0.5).collect();
    for &draught in &draught_steps {
        let (area, sub_area) = windage.calculate_area(draught, 0.);
        let bow = windage.bow_area(draught, 0.).unwrap();
        println!(
            "{:.3}: area:{:.3} sx:{:.3} mx:{:.3} sz:{:.3} mz:{:.3}  sub_area:{:.3} sx:{:.3} sz:{:.3} bow:{:.3}",
            draught,
            area.area,
            area.center_x,
            area.center_x*area.area,
            area.center_z,
            area.center_z*area.area,
            sub_area.area, 
            sub_area.center_x,
            sub_area.center_z,
            bow
        );
    }
}
