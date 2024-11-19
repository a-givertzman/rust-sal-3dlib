mod cli;

use std::time::Instant;

use clap::Parser;
use cli::{Cli, Cmd, Examples, Orientation};
use glam::dvec3;
use opencascade::{
    primitives::{Compound, Edge, IntoShape, Shape, Solid},
    workplane::Workplane,
};

fn main() {
    let args = Cli::parse();

    // grab result into one model object
    let output_model = {
        // target is usually an input model,
        // result is result model of chosen cmd
        let (target, result) = match args.command {
            Cmd::Volume {
                orientation,
                width,
                heigh,
                depth,
                input,
            } => {
                // expect model to be Shell internally
                let model = Shape::read_step(&input).expect("Failed while reading STEP");
                // plane should intersect target model
                let plane = build_plane(orientation, width, heigh, depth);
                // if two models aren't intersected, it gives nothing as result
                let under_plane = Solid::volume([&model, &plane], true).into_shape();

                // cacl some metrics:
                // - total area of result of volume operation (VO)
                // - area of plane, which is an argument of VO
                // - area of result model of VO - area of argument plane
                let (total, top_plane, valume) = calc_volumed_area(&under_plane, &plane);
                println!(
                    "AREA (sq. m.):\n- total={:.4}\n- top plane={:.4}\n- under plane solid={:.4}",
                    total, top_plane, valume
                );

                (model, under_plane)
            }
            Cmd::Intersect {
                orientation,
                heigh,
                width,
                depth,
                input,
            } => {
                let model = Shape::read_step(input).expect("Failed while reading STEP");
                let plane = build_plane(orientation, width, heigh, depth);

                #[cfg(feature = "verbose")]
                let instant = Instant::now();

                // use extended implementation of intersection
                // to take into account params of operation builder
                let result = model
                    .intersect_with_params(
                        &plane,
                        args.run_parallel,
                        args.fuzzy_value,
                        args.use_obb,
                        args.glue,
                    )
                    .into_shape();

                #[cfg(feature = "verbose")]
                dbg!(instant.elapsed());

                (model, result)
            }
            #[cfg(feature = "verbose")]
            Cmd::Example { name } => match name {
                Examples::CableBracketSelfIntersection => {
                    let model_a = examples::cable_bracket::shape();
                    let model_b = examples::cable_bracket::shape();

                    let instant = Instant::now();
                    let result = model_b.intersect_with_params(
                        &model_a,
                        args.run_parallel,
                        args.fuzzy_value,
                        args.use_obb,
                        args.glue,
                    );
                    dbg!(
                        instant.elapsed(),
                        args.run_parallel,
                        args.fuzzy_value,
                        args.use_obb,
                        args.glue
                    );

                    (model_b, result.into_shape())
                }
            },
        };

        // bake target and result
        if args.join_all {
            Compound::from_shapes([target, result]).into_shape()
        // remain only result edges
        } else if args.only_edges {
            Compound::from_shapes(result.edges().map(Edge::into_shape)).into_shape()
        // don't format result
        } else {
            result
        }
    };

    output_model
        .write_step(args.output)
        .expect("Failed while writting STEP");
}

/// Build a plane to use with boolean and volume operations.
fn build_plane(o: Orientation, w: f64, h: f64, d: f64) -> Shape {
    let (plane, dir) = match o {
        Orientation::Xy => (Workplane::xy(), dvec3(0.0, 0.0, d)),
        Orientation::Xz => (Workplane::xz(), dvec3(0.0, d, 0.0)),
        Orientation::Yz => (Workplane::yz(), dvec3(0.0, 0.0, d)),
    };

    plane.translated(dir).rect(w, h).to_face().into_shape()
}

/// Calculate areas of volume operation result.
/// It might take time due to call of boolean operation internally.
fn calc_volumed_area(shape: &Shape, top_plane: &Shape) -> (f64, f64, f64) {
    // normalize to square meters
    let normalizer = 1e6;

    let top_plane_area = shape
        .intersect(top_plane)
        .faces()
        .next()
        .unwrap()
        .surface_area();
    let total_area: f64 = shape.faces().map(|face| face.surface_area()).sum();

    (
        // total area of shape
        total_area / normalizer,
        // area of shape and plane intersection
        top_plane_area / normalizer,
        // total are - plane area
        (total_area - top_plane_area) / normalizer,
    )
}
