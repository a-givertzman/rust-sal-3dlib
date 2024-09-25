use clap::{Parser, Subcommand, ValueEnum};

use std::path::PathBuf;

#[derive(ValueEnum, Clone)]
pub enum Orientation {
    Xy,
    Xz,
    Yz,
}

#[derive(ValueEnum, Clone)]
pub enum Examples {
    /// Common of cable_bracket and keycap models
    #[clap(name = "CabBrSelfInt")]
    CableBracketSelfIntersection,
}
/// Examples:
/// - build solid by intersecting plane and hull shell model:
///   ```bash
///   cargo run -rF verbose -- volume xy 250000.0 250000.0 5000.0 ../3ds/hull_shell_centered.step
///   ```
/// - get result face joined with original model after intersection of plane and hull shell:
///   ```bash
///   cargo run -rF verbose -- \
///     --run-parallel --join-all \
///     intersect yz 200000.0 200000.0 112000.0 ../3ds/hull_shell_centered.step
///   ```
/// - run prepared example:
///   ```bash
///   cargo run -rF verbose -- --glue=2 --fuzzy-value=1e-2 example CabBrSelfInt
///   ```
#[derive(Parser, Clone)]
pub struct Cli {
    /// Output file
    #[clap(short, long, default_value = "output.step")]
    pub output: PathBuf,

    /// Join all parts
    #[clap(long, default_value_t = false, group = "view")]
    pub join_all: bool,

    /// Save only edges
    #[clap(long, default_value_t = false, group = "view")]
    pub only_edges: bool,

    #[command(subcommand)]
    pub command: Cmd,

    /// Parallel optimization
    #[clap(long)]
    pub run_parallel: bool,

    /// OBB usage
    #[clap(long)]
    pub use_obb: bool,

    /// Glue option
    #[clap(long, default_value_t = 0)]
    pub glue: u8,

    /// Fuzzy value (aka tolerance) for boolean operations
    #[clap(long, default_value_t = 1e-7)]
    pub fuzzy_value: f64,
}

#[derive(Subcommand, Clone)]
pub enum Cmd {
    /// Split shell using Volume maker
    Volume {
        orientation: Orientation,
        width: f64,
        heigh: f64,
        depth: f64,

        /// Input file path
        #[clap(value_parser = input_parser)]
        input: PathBuf,
    },

    /// Build plane intersection
    Intersect {
        orientation: Orientation,
        width: f64,
        heigh: f64,
        depth: f64,

        /// Input file path
        #[clap(value_parser = input_parser)]
        input: PathBuf,
    },

    /// Prepared examples
    #[cfg(feature = "verbose")]
    Example { name: Examples },
}

fn input_parser(arg: &str) -> anyhow::Result<PathBuf> {
    use anyhow::bail;
    use std::{os::unix::ffi::OsStrExt, str::FromStr};

    let input = PathBuf::from_str(arg)?;

    if !matches!(
        input.extension().map(OsStrExt::as_bytes),
        Some(b"stp" | b"step")
    ) {
        bail!("Supported output format is .step (.stp).");
    }

    Ok(input)
}