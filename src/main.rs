use std::{path::Path, thread, time::Duration};

use graphics::visualise::GraphicsContext;
use physics::GRAVITY;
use sdl2::event::Event;

use crate::{body::Body, vector::Vector2D};

mod body;
mod graphics;
mod physics;
mod vector;

fn main() {
    let mut context = GraphicsContext::new().unwrap();

    let bodies = Body::from_file(Path::new("examples/test.csv")).unwrap();

    for body in bodies.iter() {
        println!("{:?}", body);
    }

    context.renderBodies(&bodies).unwrap();

    'running: loop {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        context.renderBodies(&bodies).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Hello, world! Gravity: {}", GRAVITY);
}
