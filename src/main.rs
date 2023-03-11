use std::path::Path;

use graphics::visualise::GraphicsContext;
use sdl2::event::Event;
use simulate::simulate_tick;

use crate::body::Body;

mod body;
mod graphics;
mod physics;
mod simulate;
mod vector;

const TICK_TIME: f64 = 0.001;

fn main() {
    let mut context = GraphicsContext::new().unwrap();

    let mut bodies = Body::from_file(Path::new("examples/random.csv")).unwrap();

    for body in bodies.iter() {
        println!("{:?}", body);
    }

    'running: loop {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        context.render_bodies(&bodies).unwrap();
        simulate_tick(&mut bodies, TICK_TIME);
    }
}
