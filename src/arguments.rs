use std::path::Path;

use clap::Parser;

use crate::{
    body::Body,
    physics::GRAVITY,
    simulate::{SimulationState, UnitParameters, WeightUnit},
};

#[derive(Parser, Clone, Debug)]
#[command(version = "0.1.0")]
pub struct CliArguments {
    /// The input file to read the bodies from (CSV format, see examples/ or generate with generator.rs)
    input_file: String,

    /// The number of simulated tickes (of dt lenght) for which the simulation should run, infinity if not provided.
    #[arg(short, long)]
    pub passes: Option<i32>,

    /// The mass Unit
    #[arg(long, default_value_t = WeightUnit::Kilograms)]
    mass_unit: WeightUnit,

    /// The number of meters per pixel
    #[arg(long, short, default_value_t = 1.0)]
    pub distance_per_pixel: f64,

    /// The time step (dt) in seconds
    #[arg(long, default_value_t = 1000.0)]
    dt: f64,

    /// save each frame to a file at the given path
    /// the file name will be the frame number
    #[arg(long)]
    pub save_path: Option<String>,
}

impl SimulationState {
    pub fn from_cli_arguments(args: CliArguments) -> Result<SimulationState, String> {
        let bodies = Body::from_file(Path::new(&args.input_file))?;

        Ok(SimulationState {
            bodies,
            dt: args.dt,
            unit_parameters: UnitParameters {
                weight: args.mass_unit,
                meters_per_pixel: args.distance_per_pixel,
                gravity: GRAVITY,
            },
            remaining_passes: args.passes,
        })
    }
}
