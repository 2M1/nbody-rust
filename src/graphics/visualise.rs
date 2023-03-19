use std::{thread, time::Duration};

use sdl2::{
    self, event::Event, image::SaveSurface, pixels::Color, rect::Point, render::Canvas,
    surface::Surface, video::Window, EventPump,
};

use crate::{body::Body, vector::Vector2D};

pub struct GraphicsContext {
    pub canvas: Canvas<Window>,
    context: sdl2::Sdl,
    pub event_pump: EventPump,
    zoom: f64,
}

fn find_sdl_gl_driver() -> Result<u32, String> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Ok(index as u32);
        }
    }
    return Err("No OpenGL driver found".to_string());
}

impl GraphicsContext {
    pub fn new() -> Result<GraphicsContext, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let eventpump = sdl_context.event_pump()?;
        let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        let window = video_subsystem
            .window("N-body simulation", 1000, 1000)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .map_err(|err| err.to_string())?;

        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver()?)
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.present();
        canvas.clear();

        return Ok(GraphicsContext {
            canvas,
            context: sdl_context,
            event_pump: eventpump,
            zoom: 1.0,
        });
    }

    pub fn event_loop_thread(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn render_bodies(
        &mut self,
        bodies: &Vec<Body<Vector2D>>,
        meters_per_pixel: f64,
    ) -> Result<(), String> {
        self.canvas.present();
        self.canvas.clear();

        let y_axis_zero = (self.canvas.viewport().height() / 2) as i32;
        let x_axis_zero = (self.canvas.viewport().width() / 2) as i32;

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for body in bodies {
            let x = body.position.x / meters_per_pixel;
            let y = body.position.y / meters_per_pixel;

            let x = (x * self.zoom) as i32 + x_axis_zero;
            let y = (y * self.zoom) as i32 + y_axis_zero;

            self.canvas
                .draw_point(Point::new(x, y))
                .map_err(|e| e.to_string())?;
        }
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        return Ok(());
    }

    pub fn zoom_inc(&mut self, zoom: f64) {
        self.zoom += zoom;
        // println!("new zoom: {}", self.zoom);
    }

    pub fn save_image(&mut self, path: &str, number: i64) -> Result<(), String> {
        let surface = self.canvas.window().surface(&self.event_pump)?;

        let surface = unsafe { Surface::from_ll(surface.raw()) };

        surface
            .save(format!("{}/{:010}.png", path, number))
            .map_err(|e| e.to_string())?;

        return Ok(());
    }
}
