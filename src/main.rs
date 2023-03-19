use clap::Parser;
use graphics::visualise::GraphicsContext;
use sdl2::{event::Event, mouse::MouseWheelDirection};
use simulate::SimulationState;

mod arguments;
mod body;
mod graphics;
mod physics;
mod simulate;
mod vector;

/// Runs the n-body simulation
///
/// For parameters see the help message (run with --help or no arguments)
fn main() {
    let mut context = GraphicsContext::new().unwrap();

    let args = arguments::CliArguments::parse();
    let path = args.save_path.as_ref();

    let mut simulation = match SimulationState::from_cli_arguments(args.clone()) {
        Ok(sim) => sim,
        Err(msg) => {
            println!("Error parsing Input: {}", msg);
            panic!("couldn't load Simulation from provided Parameters!");
        }
    };

    let mut i = 0;
    'running: loop {
        let mut zoom_changes = 0;
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::MouseWheel {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x: _,
                    y,
                    direction,
                } => {
                    let y = (-1 as i32).pow((direction == MouseWheelDirection::Flipped) as u32) * y;
                    zoom_changes += y;
                    println!("Zoom change: {}", y);
                }
                _ => {}
            }
        }

        context.zoom_inc(zoom_changes as f64 / 10.0);
        context
            .render_bodies(&simulation.bodies, args.distance_per_pixel)
            .unwrap();

        path.map(|p| {
            context.save_image(p.as_str(), i).unwrap();
        });

        simulation.simulate_tick();
        i += 1;
    }
}
